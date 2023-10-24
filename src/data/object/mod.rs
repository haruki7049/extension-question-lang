/// Object
#[cfg(not(doctest))]
pub enum Object {
    /// Void Object
    Void,

    /// Integer value, e.g. "1"
    /// this value can be between 0 and 31.
    /// this value must express as "!???????" or "!???!!!!", so if you use INTEGER, the first Token must be '!'.
    /// e.g.
    /// ```
    /// !??????? // 0
    /// !??????! // 1
    /// !?????!? // 2
    ///  ~
    /// !??!!!!! // 31
    /// ```
    Integer(isize),

    // KEYWORD DEFINITION

    /// IF keyword used as forking process stream.
    /// this keyword need INTEGER value to judge 0 or others.
    /// 0 -> true
    /// other number -> false
    /// ```
    /// ?!!!!!!! // IF
    /// !??????? // 0
    /// 
    /// ```
    If,

    /// IFEND keyword used as express IF expresstion's end.
    IfEnd,

    /// LAMBDA keyword used as function definition.
    Lambda,

    /// PLUS keyword used as addition operation.
    /// this keyword use PN(Polish Notation).
    /// e.g.
    /// ```
    /// ?!?!!!!! // PLUS
    /// !??????! // 1
    /// !?????!? // 2
    ///  ~ return value
    /// 3
    /// ```
    Plus,

    /// MINUS keyword used as subtract operation.
    /// this keyword use PN(Polish Notation).
    /// e.g.
    /// ```
    /// ??!!!!!! // MINUS
    /// !?????!! // 3
    /// !??????! // 1
    ///  ~ return value
    /// 2
    /// ```
    Minus,

    /// ASTERISK keyword used as multiplication operation.
    /// this keyword use PN(Polish Notation).
    /// e.g.
    /// ```
    /// ?!!!!!!! // ASTERISK
    /// !?????!! // 3
    /// !?????!! // 3
    ///  ~ return value
    /// 9
    /// ```
    Asterisk,

    /// SLASH keyword used as division operation.
    /// this keyword use PN(Polish Notation).
    /// this only return quotient.
    /// e.g.
    /// ```
    /// ?!!!!!!! // ASTERISK
    /// !?????!! // 3
    /// !?????!! // 2
    ///  ~ return value
    /// 1
    /// ```
    Slash,
}

// とりあえず書いたが、命令の表を作るべきではないかとボブは訝しんだ。
