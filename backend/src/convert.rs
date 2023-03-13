trait PhonologicalRepresentation {
    fn xsampa_to_ipa(&self) -> String;
    fn ipa_to_xsampa(&self) -> String;
}

impl PhonologicalRepresentation for String {
   // check current symbol with the following symbols, continue if the added symbols are
   // valid xsampa. If a symbol that cannot be parsed as part of a xsampa representation or
   // a suprasegmental, convert the sequence to IPA
    fn xsampa_to_ipa(&self) -> String {
        //let mut output: String;
        todo!();
    }

    fn ipa_to_xsampa(&self) -> String {
        todo!();
    }
}
