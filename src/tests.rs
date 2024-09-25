use crate::{but_check, demo, ParsedText, SentimentIntensityAnalyzer, EMOJI_LEXICON, LEXICON};
use unicase::UniCase;

#[test]
fn test_lexicon() {
    assert_eq!(*LEXICON.get(&UniCase::new("feudally")).unwrap(), -0.6);
    assert_eq!(*LEXICON.get(&UniCase::new("irrationalism")).unwrap(), -1.5);
    assert_eq!(*LEXICON.get(&UniCase::new("sentimentalize")).unwrap(), 0.8);
    assert_eq!(*LEXICON.get(&UniCase::new("wisewomen")).unwrap(), 1.3);
}

#[test]
fn test_emoji_lexicon() {
    assert_eq!(*EMOJI_LEXICON.get("👽").unwrap(), "alien");
    assert_eq!(
        *EMOJI_LEXICON.get("👨🏿‍🎓").unwrap(),
        "man student: dark skin tone"
    );
    assert_eq!(
        *EMOJI_LEXICON.get("🖖🏻").unwrap(),
        "vulcan salute: light skin tone"
    );
}

#[test]
fn test_parsed_text() {
    let messy_text = "WOAH!!! ,Who? DO u Think you're?? :) :D :^(";
    let parsed_messy = ParsedText::from_text(messy_text);
    let expected_text: Vec<UniCase<&str>> =
        ["WOAH", "Who", "DO", "Think", "you\'re", ":)", ":D", ":^("]
            .iter()
            .map(|r| UniCase::new(*r))
            .collect();
    assert_eq!(parsed_messy.tokens, expected_text);
    assert!(parsed_messy.has_mixed_caps);
    assert_eq!(parsed_messy.punc_amplifier, 1.416);

    assert!(
        !ParsedText::has_mixed_caps(&ParsedText::tokenize("yeah!!! I'm aLLERGIC to ShouTING."))
    );
    assert!(
        !ParsedText::has_mixed_caps(&ParsedText::tokenize("OH MAN I LOVE SHOUTING!"))
    );
    assert!(
        ParsedText::has_mixed_caps(&ParsedText::tokenize("I guess I CAN'T MAKE UP MY MIND"))
    );
    assert!(
        ParsedText::has_mixed_caps(&ParsedText::tokenize("Hmm, yeah ME NEITHER"))
    );
}

#[test]
fn but_check_test() {
    let tokens: Vec<UniCase<&str>> = [
        "yeah", "waffles", "are", "great", "but", "have", "you", "ever", "tried", "spam",
    ]
    .iter()
    .map(|r| UniCase::new(*r))
    .collect();
    let mut sents = vec![0.5, 0.1, 0.0, 0.2, 0.6, 0.25, 0.5, 0.5, 0.5, 0.5];
    but_check(&tokens, &mut sents);
    assert_eq!(
        sents,
        vec![0.25, 0.05, 0.0, 0.1, 0.6, 0.375, 0.75, 0.75, 0.75, 0.75]
    );
}

#[test]
fn demo_test() {
    demo::run_demo();
}

#[test]
fn embedded_emoji_test() {
    let analyzer = SentimentIntensityAnalyzer::new();
    let single_emoji = "😀";
    let embedded_emoji = "heyyyy 😀 what're you up to???";
    let multiple_emoji = "woah there 😀😀😀 :) :)";
    assert_eq!(
        analyzer.append_emoji_descriptions(single_emoji),
        "grinning face"
    );
    assert_eq!(
        analyzer.append_emoji_descriptions(embedded_emoji),
        "heyyyy grinning face what're you up to???"
    );
    assert_eq!(
        analyzer.append_emoji_descriptions(multiple_emoji),
        "woah there grinning face grinning face grinning face :) :)"
    );
}

#[test]
fn test_polarity_scores() {
    let analyzer = SentimentIntensityAnalyzer::new();
    let scores = analyzer.polarity_scores("qsv is smart, handsome, and funny.");
    let tolerance = 1e-6;

    // assert_eq!(*scores.get("compound").unwrap(), 0.831632);
    // assert_eq!(*scores.get("neg").unwrap(), 0.0);
    // assert_eq!(*scores.get("neu").unwrap(), 0.0);
    // assert_eq!(*scores.get("pos").unwrap(), 1.0);

    assert!(
        (*scores.get("compound").unwrap() - 0.8316320352807864).abs() < tolerance,
        "compound score is not within tolerance"
    );
    assert!(
        (*scores.get("neg").unwrap() - 0.0).abs() < tolerance,
        "neg score is not within tolerance"
    );
    assert!(
        (*scores.get("neu").unwrap() - 0.2542372881355932).abs() < tolerance,
        "neu score is not within tolerance"
    );
    assert!(
        (*scores.get("pos").unwrap() - 0.7457627118644068).abs() < tolerance,
        "pos score is not within tolerance"
    );
}

#[test]
fn test_polarity_scores_with_negation() {
    let analyzer = SentimentIntensityAnalyzer::new();
    let scores = analyzer.polarity_scores("qsv is not smart, handsome, and funny.");
    let tolerance = 1e-6;

    // assert_eq!(*scores.get("compound").unwrap(), 0.0);
    // assert_eq!(*scores.get("neg").unwrap(), 0.0);
    // assert_eq!(*scores.get("neu").unwrap(), 0.0);
    // assert_eq!(*scores.get("pos").unwrap(), 1.0);

    assert!(
        (*scores.get("compound").unwrap() - -0.24671445723280327).abs() < tolerance,
        "compound score is not within tolerance"
    );
    assert!(
        (*scores.get("neg").unwrap() - 0.41455964703885967).abs() < tolerance,
        "neg score is not within tolerance"
    );
    assert!(
        (*scores.get("neu").unwrap() - 0.33938571186153066).abs() < tolerance,
        "neu score is not within tolerance"
    );
    assert!(
        (*scores.get("pos").unwrap() - 0.2460546410996097).abs() < tolerance,
        "pos score is not within tolerance"
    );
}
