pub trait State { }

pub enum Parsing { }

impl State for Parsing { }

pub enum Generating { }

impl State for Generating { }