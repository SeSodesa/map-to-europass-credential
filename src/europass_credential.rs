/*!
This submodule defines the Europass credential type EuropassCredential
and the types required in the construction of its fields.
*/

use chrono;

/// A set of one or more claims made by an issuer.
/// A credential is a set of one or more claims made by the same entity.
/// A verifiable credential is a tamper-evident credential that has
/// authorship that can be cryptographically verified.
/// Verifiable credentials can be used to build verifiable presentations,
/// which can also be cryptographically verified.
struct VerifiableCredential {
    /// A unique portable identifier of the credential.
    id: Identifier,
    /// The issuance date of the credential.
    issuance_date: chrono::naive::NaiveDate,
    /// The date and time the credential was digitally signed.
    issued: chrono::naive::NaiveDate,
    /// The earliest date when the information associated with
    /// the credentialSubject property became valid.
    valid_from: chrono::naive::NaiveDate,
    /// The expiration date of this credential.
    expiration_date: chrono::naive::NaiveDate,
    /// The Europass Credential carried within this verifiable credential.
    europass_credential: EuropassCredential,
}


/// A set of claims made by an issuer in Europe, using the Europass Standards.
/// A Europass credential is a set of one or more claims which may be used to
/// demonstrate that the owner has certain skills or has achieved certain
/// learning outcomes through formal, non-formal or informal learning.
struct EuropassCredential {
    /// The identifier of this Europass Credential.
    identifier: String,
    /// The type of this credential.
    /// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/credential/25831c2
    /// for details.
    credential_type: EuropassCredentialType,
    /// The full official title of the issued credential
    /// (maximum 50 characters).
    title: String,
    /// A summary of the claim or group of claims being
    /// made about a person (maximum 140 words).
    description: String,
    /// The organisaton that issued the credential and
    /// sealed it with their digital e-seal.
    issuer: String,
    /// The person about which claims are made and who owns the credential.
    credential_subject: String,
    /// The display details of the credential.
    display: String,
    /// Any digital document (PDF, JPEG or PNG format) that an issuer
    /// has attached to the Europass document.
    attachment: EuropassAttachment,
    /// The cryptographic proofs that can be used to detect tampering and
    /// verify the authorship of a credential or presentation.
    proof: Proof,
    /// A credential embedded within the credential.
    /// Smaller sub-credentials (micro-credentials),
    /// that make up this larger credential when combined.
    contains: Box<EuropassCredential>,
}

/// The Europass Standard List of Credential Types is a centrally devised
/// list of customised credential profiles that issuing organisations can
/// choose from to describe their credentials that correspond to that profile.
enum EuropassCredentialType {
    /// Represents a credential which describes that
    /// an activity has been or is being done
    LearningActivity,
    /// Represents the award of a qualification by an institution
    /// that is accredited to do so, confirmed by its inclusion in
    /// the Europass Accreditation Database.
    QualificationAward,
    /// Represents the award of a higher education qualification by
    /// an institution that is accredited to do so, confirmed by its
    /// inclusion in the Europass Accreditation Database,
    /// and including all diploma supplement information.
    DiplomaSupplement,
    /// Represents a credential which describes that the user has received a right.
    LearningEntitlement,
    /// This is the default Europass credential type.
    Generic,
}

/// A character string used to identify a resource.
/// An identifier is a character string used to uniquely identify
/// one instance of an object within an identification scheme that is managed by an agency.
/// The Identifier class is a critical aspect of the EDCI model.
/// It is used to identify persons and organisations.
/// In these cases and more, the identifier itself will be some sort of
/// alpha-numeric string but that string only has meaning if it is contextualised.
struct Identifier {
    /// Content string which is the identifier.
    /// A character string used to uniquely identify
    /// one instance of an object within an identification
    /// scheme that is managed by an agency.
    content: String,
    /// Identification of the identifier scheme.
    /// The identifier register (the managing/originating system of the identifier).
    /// This can be seen as the namespace in which the assigned identifier is unique.
    identifier_scheme_id: String,
    /// Identification of the version of the identifier scheme.
    identifier_scheme_version_id: String,
    /// Identification of the agent that manages the identifier scheme.
    /// The agent that issued the identifier. (e.g. a URI).
    identifier_scheme_agency_id: String,
    /// The name of the identifier scheme.
    identifier_scheme_name: String,
    /// The name of the agent that manages the identifier scheme.
    /// The agent that issued the identifier.
    identifier_scheme_agency_name: String,
    /// The date on which the identifier was issued.
    issued_date: chrono::naive::NaiveDate,
    /// A code used to classify the type of identifier.
    identifier_type: String,
}

/// A legal identifier is a formally issued identifier by a given
/// authorithy within a given jurisdiction.
/// The identifier has a spatial context.
struct LegalIdentifier {
    /// The identifier of the country and/or jurisdiction.
    /// Recommended RSA: MDR Countries Named Authority Lis. NUTS.
    spatial_id: String
}

/// A term from a controlled vocabulary. (a code from a code list)
/// Interoperability between data sets is increased dramatically when
/// terms from controlled vocabularies are used in favour of free text.
/// The conceptual Code data type is used wherever one or more controlled
/// vocabularies are known to exist for a particular domain of interest.
/// It is not the job of the JV/CV Vocabularies to mandate which controlled
/// vocabularies are used but we offer some guidance on how to use them.
struct Code {
    /// The term bieng described.
    target_notation: String,
    /// The identification of the controlled vocabulary
    /// (the code list). (e.g. a URI)
    target_framework_uri: String,
    /// The name of the controlled vocabulary (the code list).
    target_framework: String,
    /// The text equivalent of the code content component.
    target_name: String,
    /// A description of the target term.
    target_description: String,
    /// A portable identifier (i.e a URI) of the code.
    uri: String,
}

/// A character string (i.e. a finite set of characters)
/// generally in the form of words of a language.
struct Text {
    /// The content of the string.
    content: String,
    /// The language of the content field.
    language: EuropeanLanguage
}

/// A formatted character string (i.e. a finite set of characters)
/// generally in the form of words of a language. The character string
/// is passed/included in, and can be represented as, a (formatted) document
/// fragment (formatted) according a given mimetype (e.g. "text/plain", "text/html", etc.)
struct Note {
    /// The free text note.
    content: String,
    /// The identifier of the language used in the Content attribute.
    language: EuropeanLanguage,
    /// The identifier of the mimetype used in the Content attribute.
    format: String,
    /// The information topic this note is about.
    topic: String,
}

/// A notation (or code) is a character string according
/// a given syntax encoding scheme.
struct Notation {
    /// A notation, or its code.
    content: String,
    /// The syntax encoding scheme.
    /// A particular system of notations or classification codes.
    scheme_id: String,
}

/// A score associated with a credential.
struct Score {
    /// The score itself.
    content: f64,
    /// The identifier of the scoring scheme used in the Content attribute.
    /// Refers to the type of scoring methodology or convention.
    scoring_scheme: String,
}

/// A numeric score. Extends Score.
struct NumericScore {
    content: f64,

}

/// A textual accreditation. Extends Score.
struct TextualScore {
    content: String,
}

/// A standard measure with a unit from
/// the MDR Measurement unit Named Authority List.
struct Measure {
    /// The numeric value (i.e. measure).
    content: f64,
    /// A code indicating the type of unit measure,
    /// such as “minutes”, “hours”, “meters”, etc.
    ///
    /// Based on MDR Measurement unit Named Authority List.
    unit: MDRunit,
}

impl std::convert::TryFrom<&str> for MDRunit {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "2N"  => Ok(MDRunit::Decibel),
            "3C"  => Ok(MDRunit::Manmonth),
            "AD"  => Ok(MDRunit::Byte),
            "AMP" => Ok(MDRunit::Ampere),
            "BAR" => Ok(MDRunit::Bar),
            "BIT" => Ok(MDRunit::Bit),
            "BQL" => Ok(MDRunit::Becquerel),
            "C34" => Ok(MDRunit::Mole),
            "C45" => Ok(MDRunit::Nanometre),
            "CDL" => Ok(MDRunit::Candela),
            "CEL" => Ok(MDRunit::DegreeCelsius),
            "CMK" => Ok(MDRunit::SquareCentimetre),
            "CMQ" => Ok(MDRunit::CubicCentimetre),
            "CMT" => Ok(MDRunit::Centimetre),
            "D30" => Ok(MDRunit::TeraJoule),
            "E34" => Ok(MDRunit::GigaByte),
            "GRM" => Ok(MDRunit::Gram),
            "GTE" => Ok(MDRunit::GrossTonnage),
            "GWH" => Ok(MDRunit::GigaWattHour),
            "HAR" => Ok(MDRunit::Hectare),
            "HLT" => Ok(MDRunit::HectoLitre),
            "HTZ" => Ok(MDRunit::Hertz),
            "HUR" => Ok(MDRunit::Hour),
            "JOU" => Ok(MDRunit::Joule),
            "KEL" => Ok(MDRunit::Kelvin),
            "KGM" => Ok(MDRunit::Kilogram),
            "KMH" => Ok(MDRunit::KilometrePerHour),
            "KMK" => Ok(MDRunit::SquareKilometre),
            "KTM" => Ok(MDRunit::Kilometre),
            "KWH" => Ok(MDRunit::KiloWattHour),
            "KWT" => Ok(MDRunit::KiloWatt),
            "LH"  => Ok(MDRunit::LabourHour),
            "LTR" => Ok(MDRunit::Litre),
            "MAW" => Ok(MDRunit::MegaWatt),
            "MGM" => Ok(MDRunit::MilliGram),
            "MIN" => Ok(MDRunit::Minute),
            "MLT" => Ok(MDRunit::MilliLitre),
            "MMT" => Ok(MDRunit::MilliMetre),
            "MTK" => Ok(MDRunit::SquareMetre),
            "MTR" => Ok(MDRunit::Metre),
            "MTS" => Ok(MDRunit::MetrePerSecond),
            "NEW" => Ok(MDRunit::Newton),
            "PAL" => Ok(MDRunit::Pascal),
            "SEC" => Ok(MDRunit::Second),
            "TKM" => Ok(MDRunit::TonneKilometre),
            "TNE" => Ok(MDRunit::Tonne),
            "TOE" => Ok(MDRunit::TonneOfOilEquvalent),
            "VLT" => Ok(MDRunit::Volt),
            "WTT" => Ok(MDRunit::Watt),
            _ => Err("Could not form a Europass measure from a string…")
        }
    }
}


/// The Measurement unit authority table is a controlled vocabulary
/// listing units of measurement with their authority codes.
/// The labels and symbols are given in all official EU languages.
/// If available, the codes are based on the code list recommendation
/// N°. 20 "Codes for Units of Measure Used in International Trade"
/// (as revised periodically) maintained and published by UNECE through
/// its Center for Trade Facilitation and Electronic Business (UN/CEFACT).
/// The Measurement unit authority table is maintained by the Publications Office
/// of the European Union on the EU Vocabularies website.
enum MDRunit {
    Decibel,
    Manmonth,
    Byte,
    Ampere,
    Bar,
    Bit,
    Becquerel,
    Mole,
    Nanometre,
    Candela,
    DegreeCelsius,
    SquareCentimetre,
    CubicCentimetre,
    Centimetre,
    TeraJoule,
    GigaByte,
    Gram,
    GrossTonnage,
    GigaWattHour,
    Hectare,
    HectoLitre,
    Hertz,
    Hour,
    Joule,
    Kelvin,
    Kilogram,
    KilometrePerHour,
    SquareKilometre,
    Kilometre,
    KiloWattHour,
    KiloWatt,
    LabourHour,
    Litre,
    MegaWatt,
    MilliGram,
    Minute,
    MilliLitre,
    MilliMetre,
    SquareMetre,
    Metre,
    MetrePerSecond,
    Newton,
    Pascal,
    Second,
    TonneKilometre,
    Tonne,
    TonneOfOilEquvalent,
    Volt,
    Watt
}

/// A standard amount with a unit from
/// the MDR Vurrencies Named Authority List.
struct Amount {
    /// The numeric value (i.e. measure).
    content: f64,
    /// A code indicating the type of unit measure,
    /// such as “minutes”, “hours”, “meters”, etc.
    ///
    /// Based on MDR Currencies Named Authority List.
    unit: String,
}

/// Standard List of Europass Credential types. See
/// https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/credential
/// for details.
enum EuropassAttachment {
    PDF,
    PNG,
    JPEG,
}

/// The cryptographic proof that can be used to detect tampering and
/// verify the authorship of a credential or presentation.
struct Proof {
    /// The code indicating how to display the summary view of the credential.
    display_code: String,
    /// The background image of the credential.
    background: ImageObject
}

/// The background image of the credential,
/// embedded in its cryptographic proof.
struct ImageObject;

/// An enumeration of the official languages used in the European Union,
/// as of 2013-07-01. See https://eur-lex.europa.eu/eli/reg/1958/1(1)/2013-07-01
/// for details.
enum EuropeanLanguage {
    Bulgarian,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Irish,
    Italian,
    Latvian,
    Lithuanian,
    Maltese,
    Polish,
    Portugese,
    Romanian,
    Slovak,
    Slovene,
    Spanish,
    Swedish
}
