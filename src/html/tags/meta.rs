
pub enum MetaKind {
    // Meta custom type
    Custom(&'static str, &'static str),
    HttpEquiv(&'static str, &'static str),
    Property(&'static str, &'static str),

    // Meta "name" types
    Abstract(&'static str),
    Author(&'static str),
    Category(&'static str),
    Classification(&'static str),
    Copyright(&'static str),
    Coverage(&'static str),
    Description(&'static str),
    Designer(&'static str),
    Directory(&'static str),
    Distribution(&'static str),
    IdentifierUrl(&'static str),
    Keywords(&'static str),
    Language(&'static str),
    Owner(&'static str),
    Rating(&'static str),
    ReplyTo(&'static str),
    Revised(&'static str),
    RevisitAfter(&'static str),
    Robots(&'static str),
    Subject(&'static str),
    Summary(&'static str),
    Topic(&'static str),
    Url(&'static str),

    // Meta "http-equiv" types
    CacheControl(&'static str),
    Expires(&'static str),
    Pragma(&'static str),

    // Meta OG website types
    OgTitle(&'static str),
    OgType(&'static str),
    OgUrl(&'static str),
    OgImage(&'static str),
    OgSiteName(&'static str),
    OgDescription(&'static str),

    // Meta OG contact types
    OgEmail(&'static str),
    OgPhoneNumber(&'static str),
    OgFaxNumber(&'static str),

    // Meta OG location types
    OgLatitude(&'static str),
    OgLongitude(&'static str),
    OgStreet(&'static str),
    OgLocality(&'static str),
    OgRegion(&'static str),
    OgPostal(&'static str),
    OgCountry(&'static str),   
    
    // Meta OG Audio types
    OgAudio(&'static str),
    OgAudioTitle(&'static str),
    OgAudioArtist(&'static str),
    OgAudioAlbum(&'static str),
    OgAudioType(&'static str),

    // Meta OG Video types
    OgVideo(&'static str),
    OgVideoHeight(&'static str),
    OgVideoWidth(&'static str),
    OgVideoType(&'static str),

    // Meta Other types
    OgTypes(&'static str),
    OgPoints(&'static str),
}


#[derive(Default)]
pub struct MetaTag {
    key: String,
    value: String,
    content: String,
}

impl MetaTag {
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn content(&self) -> &str { self.content.as_ref() }

    pub fn new(key: &str, value: &str, content: &str) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            content: content.into(),
        }
    }
    
    pub fn from_kind(kind: MetaKind) -> Self {
        let (key, value, content) = match kind {
            // Meta Custom type
            MetaKind::Custom(prop, c) => ("name", prop, c),
            MetaKind::HttpEquiv(prop, c) => ("http-equiv", prop, c),
            MetaKind::Property(prop, c) => ("property", prop, c),

            // Meta "name" types
            MetaKind::Abstract(c) => ("name", "abstract", c),
            MetaKind::Author(c) => ("name", "author", c),
            MetaKind::Category(c) => ("name", "category", c),
            MetaKind::Classification(c) => ("name", "classification", c),
            MetaKind::Copyright(c) => ("name", "copyright", c),
            MetaKind::Coverage(c) => ("name", "coverage", c),
            MetaKind::Description(c) => ("name", "description", c),
            MetaKind::Designer(c) => ("name", "designer", c),
            MetaKind::Directory(c) => ("name", "directory", c),
            MetaKind::Distribution(c) => ("name", "distribution", c),
            MetaKind::IdentifierUrl(c) => ("name", "identifier-url", c),
            MetaKind::Keywords(c) => ("name", "keywords", c),
            MetaKind::Language(c) => ("name", "language", c),
            MetaKind::Owner(c) => ("name", "owner", c),
            MetaKind::Rating(c) => ("name", "rating", c),
            MetaKind::ReplyTo(c) => ("name", "reply-to", c),
            MetaKind::Revised(c) => ("name", "revised", c),
            MetaKind::RevisitAfter(c) => ("name", "revisit-after", c),
            MetaKind::Robots(c) => ("name", "robots", c),
            MetaKind::Subject(c) => ("name", "subject", c),
            MetaKind::Summary(c) => ("name", "summary", c),
            MetaKind::Topic(c) => ("name", "topic", c),
            MetaKind::Url(c) => ("name", "url", c),

            // Meta "http-equiv" types
            MetaKind::CacheControl(c) => ("http-equiv", "Cache-Control", c),
            MetaKind::Expires(c) => ("http-equiv", "Expires", c),
            MetaKind::Pragma(c) => ("http-equiv", "Pragma", c),

            // Meta OG website types
            MetaKind::OgTitle(c) => ("name", "og:title", c),
            MetaKind::OgType(c) => ("name", "og:type", c),
            MetaKind::OgUrl(c) => ("name", "og:url", c),
            MetaKind::OgImage(c) => ("name", "og:image", c),
            MetaKind::OgSiteName(c) => ("name", "og:site-name", c),
            MetaKind::OgDescription(c) => ("name", "og:description", c),

            // Meta OG contact types 
            MetaKind::OgEmail(c) => ("name", "og:email", c),
            MetaKind::OgPhoneNumber(c) => ("name", "og:phone-number", c),
            MetaKind::OgFaxNumber(c) => ("name", "og:fax-number", c),

            // Meta OG location types
            MetaKind::OgLatitude(c) => ("name", "og:latitude", c),
            MetaKind::OgLongitude(c) => ("name", "og:longitude", c),
            MetaKind::OgStreet(c) => ("name", "og:street", c),
            MetaKind::OgLocality(c) => ("name", "og:locality", c),
            MetaKind::OgRegion(c) => ("name", "og:region", c),
            MetaKind::OgPostal(c) => ("name", "og:postal", c),
            MetaKind::OgCountry(c) => ("name", "og:country", c), 

            // Meta OG Audio types
            MetaKind::OgAudio(c) => ("property", "og:audio", c),
            MetaKind::OgAudioTitle(c) => ("property", "og:audio:title", c),
            MetaKind::OgAudioArtist(c) => ("property", "og:audio:artist", c),
            MetaKind::OgAudioAlbum(c) => ("property", "og:audio:album", c),
            MetaKind::OgAudioType(c) => ("property", "og:audio:type", c),
            
            // Meta OG Video types
            MetaKind::OgVideo(c) => ("property", "og:video", c),
            MetaKind::OgVideoHeight(c) => ("property", "og:video:height", c),
            MetaKind::OgVideoWidth(c) => ("property", "og:video:width", c),
            MetaKind::OgVideoType(c) => ("property", "og:video:type", c),

            // Meta Other types
            MetaKind::OgTypes(c) => ("property", "og:types", c),
            MetaKind::OgPoints(c) => ("property", "og:points", c),
        };

        Self {
            key: key.into(),
            value: value.into(),
            content: content.into(),
        }
    }

    pub fn stringify(&self) -> String {
        format!(
            "<meta {}=\"{}\" content=\n{}\n",
            self.key, self.value, self.content,
        )
    }

}