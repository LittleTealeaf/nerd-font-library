//! Nerd Font Library
//!
//! This crate provides access to metadata and information about various Nerd Font families,
//! configurable via feature flags.

/// Represents metadata about a font family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontInfo {
    /// The display name of the font family.
    pub name: &'static str,
    /// The name of the Cargo feature flag corresponding to this font family.
    pub feature: &'static str,
}

/// A complete list of all supported font families in this library.
pub const ALL_FONTS: &[FontInfo] = &[
    FontInfo { name: "0xProto", feature: "0xproto" },
    FontInfo { name: "3270", feature: "3270" },
    FontInfo { name: "AdwaitaMono", feature: "adwaita-mono" },
    FontInfo { name: "Agave", feature: "agave" },
    FontInfo { name: "AnonymousPro", feature: "anonymous-pro" },
    FontInfo { name: "Arimo", feature: "arimo" },
    FontInfo { name: "AtkinsonHyperlegibleMono", feature: "atkinson-hyperlegible-mono" },
    FontInfo { name: "AurulentSansMono", feature: "aurulent-sans-mono" },
    FontInfo { name: "BigBlueTerminal", feature: "big-blue-terminal" },
    FontInfo { name: "BitstreamVeraSansMono", feature: "bitstream-vera-sans-mono" },
    FontInfo { name: "CascadiaCode", feature: "cascadia-code" },
    FontInfo { name: "CascadiaMono", feature: "cascadia-mono" },
    FontInfo { name: "CodeNewRoman", feature: "code-new-roman" },
    FontInfo { name: "ComicShannsMono", feature: "comic-shanns-mono" },
    FontInfo { name: "CommitMono", feature: "commit-mono" },
    FontInfo { name: "Cousine", feature: "cousine" },
    FontInfo { name: "D2Coding", feature: "d2-coding" },
    FontInfo { name: "DaddyTimeMono", feature: "daddy-time-mono" },
    FontInfo { name: "DejaVuSansMono", feature: "deja-vu-sans-mono" },
    FontInfo { name: "DepartureMono", feature: "departure-mono" },
    FontInfo { name: "DroidSansMono", feature: "droid-sans-mono" },
    FontInfo { name: "EnvyCodeR", feature: "envy-code-r" },
    FontInfo { name: "FantasqueSansMono", feature: "fantasque-sans-mono" },
    FontInfo { name: "FiraCode", feature: "fira-code" },
    FontInfo { name: "FiraMono", feature: "fira-mono" },
    FontInfo { name: "GeistMono", feature: "geist-mono" },
    FontInfo { name: "Go-Mono", feature: "go-mono" },
    FontInfo { name: "Gohu", feature: "gohu" },
    FontInfo { name: "Hack", feature: "hack" },
    FontInfo { name: "Hasklig", feature: "hasklig" },
    FontInfo { name: "HeavyData", feature: "heavy-data" },
    FontInfo { name: "Hermit", feature: "hermit" },
    FontInfo { name: "iA-Writer", feature: "ia-writer" },
    FontInfo { name: "IBMPlexMono", feature: "ibm-plex-mono" },
    FontInfo { name: "Inconsolata", feature: "inconsolata" },
    FontInfo { name: "InconsolataGo", feature: "inconsolata-go" },
    FontInfo { name: "InconsolataLGC", feature: "inconsolata-lgc" },
    FontInfo { name: "IntelOneMono", feature: "intel-one-mono" },
    FontInfo { name: "Iosevka", feature: "iosevka" },
    FontInfo { name: "IosevkaTerm", feature: "iosevka-term" },
    FontInfo { name: "IosevkaTermSlab", feature: "iosevka-term-slab" },
    FontInfo { name: "JetBrainsMono", feature: "jet-brains-mono" },
    FontInfo { name: "Lekton", feature: "lekton" },
    FontInfo { name: "LiberationMono", feature: "liberation-mono" },
    FontInfo { name: "Lilex", feature: "lilex" },
    FontInfo { name: "MartianMono", feature: "martian-mono" },
    FontInfo { name: "Meslo", feature: "meslo" },
    FontInfo { name: "Monaspace", feature: "monaspace" },
    FontInfo { name: "Monofur", feature: "monofur" },
    FontInfo { name: "Monoid", feature: "monoid" },
    FontInfo { name: "Mononoki", feature: "mononoki" },
    FontInfo { name: "MPlus", feature: "m-plus" },
    FontInfo { name: "NerdFontsSymbolsOnly", feature: "nerd-fonts-symbols-only" },
    FontInfo { name: "Noto", feature: "noto" },
    FontInfo { name: "OpenDyslexic", feature: "open-dyslexic" },
    FontInfo { name: "Overpass", feature: "overpass" },
    FontInfo { name: "ProFont", feature: "pro-font" },
    FontInfo { name: "ProggyClean", feature: "proggy-clean" },
    FontInfo { name: "Recursive", feature: "recursive" },
    FontInfo { name: "RobotoMono", feature: "roboto-mono" },
    FontInfo { name: "ShareTechMono", feature: "share-tech-mono" },
    FontInfo { name: "SourceCodePro", feature: "source-code-pro" },
    FontInfo { name: "SpaceMono", feature: "space-mono" },
    FontInfo { name: "Terminus", feature: "terminus" },
    FontInfo { name: "Tinos", feature: "tinos" },
    FontInfo { name: "Ubuntu", feature: "ubuntu" },
    FontInfo { name: "UbuntuMono", feature: "ubuntu-mono" },
    FontInfo { name: "UbuntuSans", feature: "ubuntu-sans" },
    FontInfo { name: "VictorMono", feature: "victor-mono" },
    FontInfo { name: "ZedMono", feature: "zed-mono" },
];

/// A list of the currently enabled font families based on active Cargo features.
pub const ENABLED_FONTS: &[FontInfo] = &[
    #[cfg(feature = "0xproto")]
    FontInfo { name: "0xProto", feature: "0xproto" },
    #[cfg(feature = "3270")]
    FontInfo { name: "3270", feature: "3270" },
    #[cfg(feature = "adwaita-mono")]
    FontInfo { name: "AdwaitaMono", feature: "adwaita-mono" },
    #[cfg(feature = "agave")]
    FontInfo { name: "Agave", feature: "agave" },
    #[cfg(feature = "anonymous-pro")]
    FontInfo { name: "AnonymousPro", feature: "anonymous-pro" },
    #[cfg(feature = "arimo")]
    FontInfo { name: "Arimo", feature: "arimo" },
    #[cfg(feature = "atkinson-hyperlegible-mono")]
    FontInfo { name: "AtkinsonHyperlegibleMono", feature: "atkinson-hyperlegible-mono" },
    #[cfg(feature = "aurulent-sans-mono")]
    FontInfo { name: "AurulentSansMono", feature: "aurulent-sans-mono" },
    #[cfg(feature = "big-blue-terminal")]
    FontInfo { name: "BigBlueTerminal", feature: "big-blue-terminal" },
    #[cfg(feature = "bitstream-vera-sans-mono")]
    FontInfo { name: "BitstreamVeraSansMono", feature: "bitstream-vera-sans-mono" },
    #[cfg(feature = "cascadia-code")]
    FontInfo { name: "CascadiaCode", feature: "cascadia-code" },
    #[cfg(feature = "cascadia-mono")]
    FontInfo { name: "CascadiaMono", feature: "cascadia-mono" },
    #[cfg(feature = "code-new-roman")]
    FontInfo { name: "CodeNewRoman", feature: "code-new-roman" },
    #[cfg(feature = "comic-shanns-mono")]
    FontInfo { name: "ComicShannsMono", feature: "comic-shanns-mono" },
    #[cfg(feature = "commit-mono")]
    FontInfo { name: "CommitMono", feature: "commit-mono" },
    #[cfg(feature = "cousine")]
    FontInfo { name: "Cousine", feature: "cousine" },
    #[cfg(feature = "d2-coding")]
    FontInfo { name: "D2Coding", feature: "d2-coding" },
    #[cfg(feature = "daddy-time-mono")]
    FontInfo { name: "DaddyTimeMono", feature: "daddy-time-mono" },
    #[cfg(feature = "deja-vu-sans-mono")]
    FontInfo { name: "DejaVuSansMono", feature: "deja-vu-sans-mono" },
    #[cfg(feature = "departure-mono")]
    FontInfo { name: "DepartureMono", feature: "departure-mono" },
    #[cfg(feature = "droid-sans-mono")]
    FontInfo { name: "DroidSansMono", feature: "droid-sans-mono" },
    #[cfg(feature = "envy-code-r")]
    FontInfo { name: "EnvyCodeR", feature: "envy-code-r" },
    #[cfg(feature = "fantasque-sans-mono")]
    FontInfo { name: "FantasqueSansMono", feature: "fantasque-sans-mono" },
    #[cfg(feature = "fira-code")]
    FontInfo { name: "FiraCode", feature: "fira-code" },
    #[cfg(feature = "fira-mono")]
    FontInfo { name: "FiraMono", feature: "fira-mono" },
    #[cfg(feature = "geist-mono")]
    FontInfo { name: "GeistMono", feature: "geist-mono" },
    #[cfg(feature = "go-mono")]
    FontInfo { name: "Go-Mono", feature: "go-mono" },
    #[cfg(feature = "gohu")]
    FontInfo { name: "Gohu", feature: "gohu" },
    #[cfg(feature = "hack")]
    FontInfo { name: "Hack", feature: "hack" },
    #[cfg(feature = "hasklig")]
    FontInfo { name: "Hasklig", feature: "hasklig" },
    #[cfg(feature = "heavy-data")]
    FontInfo { name: "HeavyData", feature: "heavy-data" },
    #[cfg(feature = "hermit")]
    FontInfo { name: "Hermit", feature: "hermit" },
    #[cfg(feature = "ia-writer")]
    FontInfo { name: "iA-Writer", feature: "ia-writer" },
    #[cfg(feature = "ibm-plex-mono")]
    FontInfo { name: "IBMPlexMono", feature: "ibm-plex-mono" },
    #[cfg(feature = "inconsolata")]
    FontInfo { name: "Inconsolata", feature: "inconsolata" },
    #[cfg(feature = "inconsolata-go")]
    FontInfo { name: "InconsolataGo", feature: "inconsolata-go" },
    #[cfg(feature = "inconsolata-lgc")]
    FontInfo { name: "InconsolataLGC", feature: "inconsolata-lgc" },
    #[cfg(feature = "intel-one-mono")]
    FontInfo { name: "IntelOneMono", feature: "intel-one-mono" },
    #[cfg(feature = "iosevka")]
    FontInfo { name: "Iosevka", feature: "iosevka" },
    #[cfg(feature = "iosevka-term")]
    FontInfo { name: "IosevkaTerm", feature: "iosevka-term" },
    #[cfg(feature = "iosevka-term-slab")]
    FontInfo { name: "IosevkaTermSlab", feature: "iosevka-term-slab" },
    #[cfg(feature = "jet-brains-mono")]
    FontInfo { name: "JetBrainsMono", feature: "jet-brains-mono" },
    #[cfg(feature = "lekton")]
    FontInfo { name: "Lekton", feature: "lekton" },
    #[cfg(feature = "liberation-mono")]
    FontInfo { name: "LiberationMono", feature: "liberation-mono" },
    #[cfg(feature = "lilex")]
    FontInfo { name: "Lilex", feature: "lilex" },
    #[cfg(feature = "martian-mono")]
    FontInfo { name: "MartianMono", feature: "martian-mono" },
    #[cfg(feature = "meslo")]
    FontInfo { name: "Meslo", feature: "meslo" },
    #[cfg(feature = "monaspace")]
    FontInfo { name: "Monaspace", feature: "monaspace" },
    #[cfg(feature = "monofur")]
    FontInfo { name: "Monofur", feature: "monofur" },
    #[cfg(feature = "monoid")]
    FontInfo { name: "Monoid", feature: "monoid" },
    #[cfg(feature = "mononoki")]
    FontInfo { name: "Mononoki", feature: "mononoki" },
    #[cfg(feature = "m-plus")]
    FontInfo { name: "MPlus", feature: "m-plus" },
    #[cfg(feature = "nerd-fonts-symbols-only")]
    FontInfo { name: "NerdFontsSymbolsOnly", feature: "nerd-fonts-symbols-only" },
    #[cfg(feature = "noto")]
    FontInfo { name: "Noto", feature: "noto" },
    #[cfg(feature = "open-dyslexic")]
    FontInfo { name: "OpenDyslexic", feature: "open-dyslexic" },
    #[cfg(feature = "overpass")]
    FontInfo { name: "Overpass", feature: "overpass" },
    #[cfg(feature = "pro-font")]
    FontInfo { name: "ProFont", feature: "pro-font" },
    #[cfg(feature = "proggy-clean")]
    FontInfo { name: "ProggyClean", feature: "proggy-clean" },
    #[cfg(feature = "recursive")]
    FontInfo { name: "Recursive", feature: "recursive" },
    #[cfg(feature = "roboto-mono")]
    FontInfo { name: "RobotoMono", feature: "roboto-mono" },
    #[cfg(feature = "share-tech-mono")]
    FontInfo { name: "ShareTechMono", feature: "share-tech-mono" },
    #[cfg(feature = "source-code-pro")]
    FontInfo { name: "SourceCodePro", feature: "source-code-pro" },
    #[cfg(feature = "space-mono")]
    FontInfo { name: "SpaceMono", feature: "space-mono" },
    #[cfg(feature = "terminus")]
    FontInfo { name: "Terminus", feature: "terminus" },
    #[cfg(feature = "tinos")]
    FontInfo { name: "Tinos", feature: "tinos" },
    #[cfg(feature = "ubuntu")]
    FontInfo { name: "Ubuntu", feature: "ubuntu" },
    #[cfg(feature = "ubuntu-mono")]
    FontInfo { name: "UbuntuMono", feature: "ubuntu-mono" },
    #[cfg(feature = "ubuntu-sans")]
    FontInfo { name: "UbuntuSans", feature: "ubuntu-sans" },
    #[cfg(feature = "victor-mono")]
    FontInfo { name: "VictorMono", feature: "victor-mono" },
    #[cfg(feature = "zed-mono")]
    FontInfo { name: "ZedMono", feature: "zed-mono" },
];

/// Returns `true` if the font family is enabled by its feature flag.
pub fn is_enabled(font_name: &str) -> bool {
    ENABLED_FONTS.iter().any(|f| f.name.eq_ignore_ascii_case(font_name) || f.feature.eq_ignore_ascii_case(font_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_fonts_list() {
        assert!(!ALL_FONTS.is_empty());
        assert_eq!(ALL_FONTS.len(), 70);
    }

    #[test]
    fn test_enabled_fonts() {
        // By default, no features are enabled (unless compiled with --all-features)
        // If a feature is active, its font should be in ENABLED_FONTS
        for font in ENABLED_FONTS {
            assert!(is_enabled(font.name));
            assert!(is_enabled(font.feature));
        }

        if cfg!(feature = "all") {
            assert_eq!(ENABLED_FONTS.len(), ALL_FONTS.len());
        }
    }
}
