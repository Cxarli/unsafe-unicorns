#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
	Instant,
	Upgrade,
	Downgrade,
	Magic,
    Unicorn(Unicorn),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unicorn {
	Baby,
	Basic,
	Magical,
}
