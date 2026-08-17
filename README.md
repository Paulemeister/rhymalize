# IPA Utils

Provides a `ipa_utils` crate for parsing IPA.

- Parse UTF-8 strings into IPA characters
- Create syllables from IPA using language specific rules
    - Syllables consist of onset, nucleus and coda
- Provides several text to IPA converters
    - fetching through `en.wiktionary.org/w/api` API
    - local lookup through json file
- Lyrics fetching through `genius.com` API 


# Rhymalize

Provides a GUI using the `iced` crate.

- Searching of lyrics through `genius.com`
- Text to IPA conversion
- Simple rhyme analysis based on matching nucleus
    - Highlighting of close rhyming syllables.
    - Highlighting of rhyming syllables on hover
