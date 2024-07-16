pub struct VariableDeclartionToken {
    /// VariableDeclartionToken
    /// for the simple variable declaration
    /// e.g. public const int a = 10;
    /// name : a
    /// value : 10
    /// declartion_type : int
    /// prefix : public, const
    /// raw_content : "public const int a = 10;"
    pub name: String,
    pub value: String,
    pub declartion_type: String,
    pub prefix: Vec<String>,
    pub raw_content: String,
}
