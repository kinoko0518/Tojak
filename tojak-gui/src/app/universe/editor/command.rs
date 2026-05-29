use winnow::{
    Parser,
    ascii::{multispace0, space0},
    combinator::{alt, delimited, eof, fail, opt, repeat, separated, terminated},
    token::take_while,
};

pub struct ConditonedTransition<'s> {
    pub condition: Vec<(&'s str, &'s str)>,
    pub branch: Vec<(Vec<bool>, &'s str)>,
    pub wildcase: Option<&'s str>,
}

pub enum ParagraphKind<'s> {
    // アトムの関係のmutate
    /// ```text
    /// :Atom <- Atom
    /// ```
    Get((&'s str, &'s str)),
    /// ```text
    /// :Atom -> Atom
    /// ```
    Lost((&'s str, &'s str)),
    /// ```text
    /// :Atom -?- Atom
    /// ```
    Both((&'s str, &'s str)),
    /// ```text
    /// :Atom -!- Atom
    /// ```
    Nether((&'s str, &'s str)),

    // アトムの関係のassert
    /// ```text
    /// :Atom.Atom
    /// ```
    AssertSome((&'s str, &'s str)),
    /// ```text
    /// :Atom.Atom?
    /// ```
    AssertSomative((&'s str, &'s str)),
    /// ```text
    /// :Atom.Atom!
    /// ```
    AssertNone((&'s str, &'s str)),
    /// ```text
    /// :Atom.Atom!?
    /// ```
    AssertNonative((&'s str, &'s str)),

    // 遷移
    /// ```text
    /// > path/to/file
    /// ```
    UnconditionedTransition(&'s str),
    /// **単一のアトムを条件とする場合**
    /// ```text
    /// > Atom.Atom?
    /// | Some => path/to/file
    /// | None => path/to/file
    /// ```
    ///
    /// **複数のアトムを条件とする場合**
    /// ```text
    /// > (Atom.Atom, Atom.Atom)?
    /// | (Some, Some) => path/to/file
    /// | (Some, None) => path/to/file
    /// | _ => path/to/file
    /// ```
    ConditonedTransition(ConditonedTransition<'s>),

    Otherwise(&'s str),
}

type PResult<T> = Result<T, ()>;

fn ident<'a>(i: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(i)
}

fn path<'a>(i: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., |c: char| !c.is_whitespace()).parse_next(i)
}

fn parse_mutation<'a>(i: &mut &'a str) -> PResult<ParagraphKind<'a>> {
    let _ = delimited(multispace0, ':', space0).parse_next(i)?;
    let left = ident.parse_next(i)?;
    let op = delimited(multispace0, alt(("<-", "->", "-?-", "-!-")), multispace0).parse_next(i)?;
    let right = ident.parse_next(i)?;

    match op {
        "<-" => Ok(ParagraphKind::Get((left, right))),
        "->" => Ok(ParagraphKind::Lost((left, right))),
        "-?-" => Ok(ParagraphKind::Both((left, right))),
        "-!-" => Ok(ParagraphKind::Nether((left, right))),
        _ => unreachable!(),
    }
}

fn parse_assertion<'a>(i: &mut &'a str) -> PResult<ParagraphKind<'a>> {
    let _ = delimited(multispace0, ':', space0).parse_next(i)?;
    let left = ident.parse_next(i)?;
    let _ = delimited(multispace0, '.', multispace0).parse_next(i)?;
    let right = ident.parse_next(i)?;

    let suffix = delimited(multispace0, alt(("!?", "?", "!", "")), multispace0).parse_next(i)?;

    match suffix {
        "" => Ok(ParagraphKind::AssertSome((left, right))),
        "?" => Ok(ParagraphKind::AssertSomative((left, right))),
        "!" => Ok(ParagraphKind::AssertNone((left, right))),
        "!?" => Ok(ParagraphKind::AssertNonative((left, right))),
        _ => unreachable!(),
    }
}

/// 条件付き遷移のヘッダ部分
fn condition_header<'a>(i: &mut &'a str) -> PResult<Vec<(&'a str, &'a str)>> {
    fn ownership<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
        (ident, delimited(multispace0, '.', multispace0), ident)
            .map(|(left, _, right)| (left, right))
            .parse_next(input)
    }

    let _ = delimited(multispace0, '>', space0).parse_next(i)?;

    let conds = alt((
        delimited(
            '(',
            separated(1.., delimited(space0, ownership, space0), ','),
            ')',
        ),
        ownership.map(|s| vec![s]),
    ))
    .parse_next(i)?;
    let _ = delimited(space0, '?', space0).parse_next(i)?;

    Ok(conds)
}

/// 条件分岐のパターン部分 (`(Some, None)` または `Some` など)
fn branch_pattern<'a>(i: &mut &'a str) -> PResult<Vec<&'a str>> {
    alt((
        delimited(
            '(',
            separated(1.., delimited(space0, ident, space0), ','),
            ')',
        ),
        ident.map(|s| vec![s]),
    ))
    .parse_next(i)
}

/// 通常の条件分岐 (`| (Some, None) => path/to/file`)
fn normal_branch<'a>(i: &mut &'a str) -> PResult<(Vec<&'a str>, &'a str)> {
    let _ = delimited(multispace0, '|', space0).parse_next(i)?;
    let pat = branch_pattern.parse_next(i)?;

    // ワイルドカード (`_`) だった場合は通常ブランチとしては失敗させる
    if pat.len() == 1 && pat[0] == "_" {
        return fail.parse_next(i);
    }

    let _ = delimited(space0, "=>", space0).parse_next(i)?;
    let p = path.parse_next(i)?;
    Ok((pat, p))
}

/// ワイルドカードの分岐 (`| _ => path/to/file`)
fn wild_branch<'a>(i: &mut &'a str) -> PResult<&'a str> {
    let _ = delimited(multispace0, '|', space0).parse_next(i)?;
    let _ = '_'.parse_next(i)?;
    let _ = delimited(space0, "=>", space0).parse_next(i)?;
    let p = path.parse_next(i)?;
    Ok(p)
}

/// 条件付き遷移の全体をパース
fn parse_conditioned<'a>(i: &mut &'a str) -> PResult<ParagraphKind<'a>> {
    let conds = condition_header.parse_next(i)?;
    let branches: Vec<_> = repeat(
        0..,
        normal_branch.verify_map(|branch| {
            let cond: Option<Vec<bool>> = branch
                .0
                .into_iter()
                .map(|pat| {
                    if pat == "Some" {
                        Some(true)
                    } else if pat == "None" {
                        Some(false)
                    } else {
                        None
                    }
                })
                .collect();
            cond.map(|c| (c, branch.1))
        }),
    )
    .parse_next(i)?;
    let wild = opt(wild_branch).parse_next(i)?;

    Ok(ParagraphKind::ConditonedTransition(ConditonedTransition {
        condition: conds,
        branch: branches,
        wildcase: wild,
    }))
}

/// 無条件遷移をパース
fn parse_unconditioned<'a>(i: &mut &'a str) -> PResult<ParagraphKind<'a>> {
    let _ = delimited(multispace0, '>', space0).parse_next(i)?;
    let p = path.parse_next(i)?;
    Ok(ParagraphKind::UnconditionedTransition(p))
}

/// トップレベルのAPI（これを利用してテキストを分類します）
pub fn parse_paragraph_kind<'a>(input: &'a str) -> ParagraphKind<'a> {
    // 前後の空白をトリム
    let mut i = input.trim();

    // terminated(..., eof) で文字列の末尾まで完全に一致するかをチェックします
    let res = alt((
        terminated(parse_mutation, eof),
        terminated(parse_assertion, eof),
        terminated(parse_conditioned, eof),
        terminated(parse_unconditioned, eof),
    ))
    .parse_next(&mut i);

    match res {
        Ok(kind) => kind,
        // 上記のフォーマットのいずれにも完全一致しなければフォールバック
        Err(_) => ParagraphKind::Otherwise(input),
    }
}
