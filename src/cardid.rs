#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardID {
    Baby, // 2019.12.24
    Basic, // 2019.12.24
    BrokenStable, // 2019.12.24
    RainbowAura, // 2019.12.24
    ShakeUp, // 2019.12.24
    NarwhalTorpedo, // 2019.12.24
    Slowdown, // 2019.12.24
    NannyCam, // 2019.12.24
    SadisticRitual, // 2019.12.24

    GinormousUnicorn, // 2019.12.25
    BarbedWire, // 2019.12.25

    UnicornPhoenix, // 2019.12.26
    TinyStable, // 2019.12.26

    GoodDeal, // 2019.12.27
    BlatantThievery, // 2019.12.27
    TargetedDestruction, // 2019.12.27

    // 2019.12.28/29/30 was mostly spent refactoring to make adding new cards easier

    UnicornLasso, // 2019.12.29

    SeductiveUnicorn, // 2019.12.30
    UnicornPoison, // 2019.12.30
    TwoForOne, // 2019.12.30

    GlitterBomb, // 2019.12.31
    SummoningRitual, // 2019.12.31
    ExtraTail, // 2019.12.31
    RainbowMane, // 2019.12.31
    ResetButton, // 2019.12.31

    UnfairBargain, // 2020.01.01
}