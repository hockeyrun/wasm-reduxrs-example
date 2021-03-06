use crate::theme::{new_theme, DarkTheme, LightTheme, ThemeMode};

#[test]
pub fn test_themes() {
    let dark_theme = new_theme::<DarkTheme>(ThemeMode::DARK);
    assert_eq!(dark_theme.mode(), ThemeMode::DARK);

    let light_theme = new_theme::<LightTheme>(ThemeMode::LIGHT);
    assert_eq!(light_theme.mode(), ThemeMode::LIGHT);
}
