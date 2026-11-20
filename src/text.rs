//! Text helpers: translation, placeholders and number formats.
use mod_api::ClientHost;

/// `key` in the player's language, or `fallback` in words when the lang has no such key.
pub fn translated(host: &dyn ClientHost, key: &str, fallback: &str) -> String {
    host.translate(key).unwrap_or_else(|| words(fallback))
}

/// The name of a registry id under a lang prefix: `("entity", "minecraft:zombie")` → "Zombie".
pub fn id_name(host: &dyn ClientHost, prefix: &str, id: &str) -> String {
    let (namespace, path) = id.split_once(':').unwrap_or(("minecraft", id));
    translated(host, &format!("{prefix}.{namespace}.{path}"), path)
}

/// One of this mod's lang lines, its `%s` replaced by `args` in order.
pub fn tr(host: &dyn ClientHost, key: &str, args: &[&str]) -> String {
    let template = host.translate(key).unwrap_or_else(|| key.to_owned());
    fill(&template, args)
}

/// `template` with each `%s` replaced by the next of `args`.
pub fn fill(template: &str, args: &[&str]) -> String {
    let mut out = String::new();
    let mut args = args.iter();
    let mut parts = template.split("%s");
    if let Some(first) = parts.next() {
        out.push_str(first);
    }
    for part in parts {
        out.push_str(args.next().copied().unwrap_or(""));
        out.push_str(part);
    }
    out
}

/// `snake_case` → "Snake Case": a mod's display name from its namespace, as Jade shows it.
pub fn words(id: &str) -> String {
    id.split(['_', '-', '/'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_uppercase().chain(c).collect::<String>())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A value with at most one decimal: 20 → "20", 7.5 → "7.5".
pub fn number(value: f32) -> String {
    let tenths = (value * 10.0).round() / 10.0;
    if tenths.fract() == 0.0 {
        format!("{tenths:.0}")
    } else {
        format!("{tenths:.1}")
    }
}

/// A fraction as a whole percentage: 0.574 → "57%".
pub fn percent(fraction: f32) -> String {
    format!("{}%", (fraction.clamp(0.0, 1.0) * 100.0) as u32)
}

/// Game ticks as minutes and seconds: 2500 → "2:05".
pub fn duration(ticks: i32) -> String {
    let seconds = ticks.max(0) / 20;
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

/// An effect level as Roman numerals, from amplifier 0 → "I".
pub fn roman(amplifier: i32) -> String {
    const NUMERALS: [&str; 10] = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
    usize::try_from(amplifier)
        .ok()
        .and_then(|i| NUMERALS.get(i))
        .map_or_else(|| (amplifier + 1).to_string(), |s| (*s).to_owned())
}

/// A note block's note (0 to 24) as its name: 0 → "F♯".
pub fn note_name(note: u32) -> &'static str {
    const NAMES: [&str; 12] = [
        "F\u{266F}", "G", "G\u{266F}", "A", "A\u{266F}", "B", "C", "C\u{266F}", "D", "D\u{266F}",
        "E", "F",
    ];
    NAMES[(note % 12) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_capitalises_each_part() {
        assert_eq!(words("minecraft"), "Minecraft");
        assert_eq!(words("piers_farm"), "Piers Farm");
    }

    #[test]
    fn numbers_keep_one_decimal() {
        assert_eq!(number(20.0), "20");
        assert_eq!(number(7.5), "7.5");
        assert_eq!(number(3.333), "3.3");
    }

    #[test]
    fn fill_replaces_placeholders_in_order() {
        assert_eq!(fill("Growth: %s", &["57%"]), "Growth: 57%");
        assert_eq!(fill("Mature", &[]), "Mature");
        assert_eq!(fill("%s / %s", &["1"]), "1 / ");
    }

    #[test]
    fn durations_and_percentages() {
        assert_eq!(duration(2500), "2:05");
        assert_eq!(duration(-5), "0:00");
        assert_eq!(percent(0.574), "57%");
        assert_eq!(percent(1.5), "100%");
    }

    #[test]
    fn roman_and_notes() {
        assert_eq!(roman(0), "I");
        assert_eq!(roman(3), "IV");
        assert_eq!(roman(20), "21");
        assert_eq!(note_name(0), "F\u{266F}");
        assert_eq!(note_name(6), "C");
        assert_eq!(note_name(24), "F\u{266F}");
    }
}
