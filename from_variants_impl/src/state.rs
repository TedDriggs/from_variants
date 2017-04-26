pub trait State { }

#[allow(dead_code)]
pub enum Parsing { }

impl State for Parsing { }

pub enum Generating { }

impl State for Generating { }