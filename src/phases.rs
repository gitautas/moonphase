use std::fmt;

pub enum MoonPhase {
    NewMoon = 0,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

impl fmt::Display for MoonPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            MoonPhase::NewMoon => "New Moon",
            MoonPhase::WaxingCrescent => "Waxing Crescent",
            MoonPhase::FirstQuarter => "First Quarter",
            MoonPhase::WaxingGibbous => "Waxing Gibbous",
            MoonPhase::FullMoon => "Full Moon",
            MoonPhase::WaningGibbous => "Waning Gibbous",
            MoonPhase::LastQuarter => "Last Quarter",
            MoonPhase::WaningCrescent => "Waning Crescent",
        };
        write!(f, "{}", s)
    }
}

impl From<u8> for MoonPhase {
    fn from(i: u8) -> Self {
        return match i {
            0 => MoonPhase::NewMoon,
            1 => MoonPhase::WaxingCrescent,
            2 => MoonPhase::FirstQuarter,
            3 => MoonPhase::WaxingGibbous,
            4 => MoonPhase::FullMoon,
            5 => MoonPhase::WaningGibbous,
            6 => MoonPhase::LastQuarter,
            7 => MoonPhase::WaningCrescent,
            8 => MoonPhase::NewMoon,
            _ => panic!(""),
        };
    }
}

impl MoonPhase {
    #[rustfmt::skip]
    pub fn ascii(&self) -> &str {
        return match self {
            MoonPhase::NewMoon =>
            {
"        _..._
      .:::::::.
     :::::::::::
     :::::::::::
     `:::::::::'
       `':::''  "
            }
            MoonPhase::WaxingCrescent =>
            {
"        _..._
      .::::. `.
     :::::::.  :
     ::::::::  :
     `::::::' .'
       `'::'-'  "
            }
            MoonPhase::FirstQuarter => {
"        _..._
      .::::  `.
     ::::::    :
     ::::::    :
     `:::::   .'
       `'::.-'  "
            }
            MoonPhase::WaxingGibbous => {
"        _..._
      .::'   `.
     :::       :
     :::       :
     `::.     .'
       `':..-'  "
            }
            MoonPhase::FullMoon => {
"        _..._
      .'     `.
     :         :
     :         :
     `.       .'
       `-...-'  "
            }
            MoonPhase::WaningGibbous => {
"        _..._
      .'   `::.
     :       :::
     :       :::
     `.     .::'
       `-..:''  "
            }
            MoonPhase::LastQuarter => {
"        _..._
      .'  ::::.
     :    ::::::
     :    ::::::
     `.   :::::'
       `-.::''  "
            }
            MoonPhase::WaningCrescent => {
"        _..._
      .' .::::.
     :  ::::::::
     :  ::::::::
     `. '::::::'
        `-.::'' "
            }
        };
    }
}
