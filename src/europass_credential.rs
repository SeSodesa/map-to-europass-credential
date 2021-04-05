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
    /// Has to be a valid URI.
    id: Identifier,
    /// The issuance date of the credential.
    issuance_date: chrono::naive::NaiveDateTime,
    /// The date and time the credential was digitally signed.
    issued: chrono::naive::NaiveDateTime,
    /// The earliest date when the information associated with
    /// the credentialSubject property became valid.
    valid_from: chrono::naive::NaiveDateTime,
    /// The expiration date of this credential.
    expiration_date: chrono::naive::NaiveDateTime,
    /// The Europass Credential carried within this verifiable credential.
    europass_credential: EuropassCredential,
}


/// A set of claims made by an issuer in Europe, using the Europass Standards.
/// A Europass credential is a set of one or more claims which may be used to
/// demonstrate that the owner has certain skills or has achieved certain
/// learning outcomes through formal, non-formal or informal learning.
struct EuropassCredential {
    /// The identifier of this Europass Credential.
    identifier: Identifier,
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
/// the MDR Currencies Named Authority List.
struct Amount {
    /// The numeric value (i.e. measure).
    content: f64,
    /// A code indicating the currency the content field is in.
    /// Based on MDR Currencies Named Authority List.
    unit: MDRcurrency,
}

/// The Currency authority table is a controlled vocabulary that lists concepts associated with currencies and currency subunits.
/// The concepts included are correlated with the ISO 4217 international standard.
/// The Currency authority table is updated based on the stakeholders’ needs.
/// Contributions to it are accepted following a review made by EU Vocabularies team of the Publications Office of the EU.
/// The Currency authority table is maintained by the Publications Office of the European Union on the EU Vocabularies website.
/// See http://publications.europa.eu/resource/authority/currency.
///
/// Can only be constructed from a known MDR currency code.
struct MDRcurrency {
    code: String
}

// Implement std::convert::TryFrom to disallow unknown currency codes.
impl std::convert::TryFrom<&str> for MDRcurrency {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ADP" => Ok(MDRcurrency {code:String::from(value)}),
            "AED" => Ok(MDRcurrency {code:String::from(value)}),
            "AFA" => Ok(MDRcurrency {code:String::from(value)}),
            "AFN" => Ok(MDRcurrency {code:String::from(value)}),
            "ALK" => Ok(MDRcurrency {code:String::from(value)}),
            "ALL" => Ok(MDRcurrency {code:String::from(value)}),
            "AMD" => Ok(MDRcurrency {code:String::from(value)}),
            "ANG" => Ok(MDRcurrency {code:String::from(value)}),
            "AOA" => Ok(MDRcurrency {code:String::from(value)}),
            "AON" => Ok(MDRcurrency {code:String::from(value)}),
            "AOR" => Ok(MDRcurrency {code:String::from(value)}),
            "ARA" => Ok(MDRcurrency {code:String::from(value)}),
            "ARM" => Ok(MDRcurrency {code:String::from(value)}),
            "ARP" => Ok(MDRcurrency {code:String::from(value)}),
            "ARS" => Ok(MDRcurrency {code:String::from(value)}),
            "ATS" => Ok(MDRcurrency {code:String::from(value)}),
            "AUD" => Ok(MDRcurrency {code:String::from(value)}),
            "AWG" => Ok(MDRcurrency {code:String::from(value)}),
            "AZM" => Ok(MDRcurrency {code:String::from(value)}),
            "AZN" => Ok(MDRcurrency {code:String::from(value)}),
            "BAM" => Ok(MDRcurrency {code:String::from(value)}),
            "BBD" => Ok(MDRcurrency {code:String::from(value)}),
            "BDT" => Ok(MDRcurrency {code:String::from(value)}),
            "BEF" => Ok(MDRcurrency {code:String::from(value)}),
            "BGJ" => Ok(MDRcurrency {code:String::from(value)}),
            "BGK" => Ok(MDRcurrency {code:String::from(value)}),
            "BGL" => Ok(MDRcurrency {code:String::from(value)}),
            "BGN" => Ok(MDRcurrency {code:String::from(value)}),
            "BHD" => Ok(MDRcurrency {code:String::from(value)}),
            "BIF" => Ok(MDRcurrency {code:String::from(value)}),
            "BMD" => Ok(MDRcurrency {code:String::from(value)}),
            "BND" => Ok(MDRcurrency {code:String::from(value)}),
            "BOB" => Ok(MDRcurrency {code:String::from(value)}),
            "BOP" => Ok(MDRcurrency {code:String::from(value)}),
            "BOV" => Ok(MDRcurrency {code:String::from(value)}),
            "BRB" => Ok(MDRcurrency {code:String::from(value)}),
            "BRC" => Ok(MDRcurrency {code:String::from(value)}),
            "BRE" => Ok(MDRcurrency {code:String::from(value)}),
            "BRL" => Ok(MDRcurrency {code:String::from(value)}),
            "BRN" => Ok(MDRcurrency {code:String::from(value)}),
            "BRR" => Ok(MDRcurrency {code:String::from(value)}),
            "BRZ" => Ok(MDRcurrency {code:String::from(value)}),
            "BSD" => Ok(MDRcurrency {code:String::from(value)}),
            "BTN" => Ok(MDRcurrency {code:String::from(value)}),
            "BWP" => Ok(MDRcurrency {code:String::from(value)}),
            "BYN" => Ok(MDRcurrency {code:String::from(value)}),
            "BYR" => Ok(MDRcurrency {code:String::from(value)}),
            "BZD" => Ok(MDRcurrency {code:String::from(value)}),
            "CAD" => Ok(MDRcurrency {code:String::from(value)}),
            "CDF" => Ok(MDRcurrency {code:String::from(value)}),
            "CHE" => Ok(MDRcurrency {code:String::from(value)}),
            "CHF" => Ok(MDRcurrency {code:String::from(value)}),
            "CHW" => Ok(MDRcurrency {code:String::from(value)}),
            "CLE" => Ok(MDRcurrency {code:String::from(value)}),
            "CLF" => Ok(MDRcurrency {code:String::from(value)}),
            "CLP" => Ok(MDRcurrency {code:String::from(value)}),
            "CNY" => Ok(MDRcurrency {code:String::from(value)}),
            "COP" => Ok(MDRcurrency {code:String::from(value)}),
            "COU" => Ok(MDRcurrency {code:String::from(value)}),
            "CRC" => Ok(MDRcurrency {code:String::from(value)}),
            "CSJ" => Ok(MDRcurrency {code:String::from(value)}),
            "CSK" => Ok(MDRcurrency {code:String::from(value)}),
            "CUC" => Ok(MDRcurrency {code:String::from(value)}),
            "CUP" => Ok(MDRcurrency {code:String::from(value)}),
            "CVE" => Ok(MDRcurrency {code:String::from(value)}),
            "CYP" => Ok(MDRcurrency {code:String::from(value)}),
            "CZK" => Ok(MDRcurrency {code:String::from(value)}),
            "DDM" => Ok(MDRcurrency {code:String::from(value)}),
            "DEM" => Ok(MDRcurrency {code:String::from(value)}),
            "DJF" => Ok(MDRcurrency {code:String::from(value)}),
            "DKK" => Ok(MDRcurrency {code:String::from(value)}),
            "DOP" => Ok(MDRcurrency {code:String::from(value)}),
            "DZD" => Ok(MDRcurrency {code:String::from(value)}),
            "ECS" => Ok(MDRcurrency {code:String::from(value)}),
            "EEK" => Ok(MDRcurrency {code:String::from(value)}),
            "EGP" => Ok(MDRcurrency {code:String::from(value)}),
            "ERN" => Ok(MDRcurrency {code:String::from(value)}),
            "ESP" => Ok(MDRcurrency {code:String::from(value)}),
            "ETB" => Ok(MDRcurrency {code:String::from(value)}),
            "EUR" => Ok(MDRcurrency {code:String::from(value)}),
            "FIM" => Ok(MDRcurrency {code:String::from(value)}),
            "FJD" => Ok(MDRcurrency {code:String::from(value)}),
            "FKP" => Ok(MDRcurrency {code:String::from(value)}),
            "FRF" => Ok(MDRcurrency {code:String::from(value)}),
            "GBP" => Ok(MDRcurrency {code:String::from(value)}),
            "GEL" => Ok(MDRcurrency {code:String::from(value)}),
            "GHC" => Ok(MDRcurrency {code:String::from(value)}),
            "GHS" => Ok(MDRcurrency {code:String::from(value)}),
            "GIP" => Ok(MDRcurrency {code:String::from(value)}),
            "GMD" => Ok(MDRcurrency {code:String::from(value)}),
            "GNE" => Ok(MDRcurrency {code:String::from(value)}),
            "GNF" => Ok(MDRcurrency {code:String::from(value)}),
            "GQE" => Ok(MDRcurrency {code:String::from(value)}),
            "GRD" => Ok(MDRcurrency {code:String::from(value)}),
            "GTQ" => Ok(MDRcurrency {code:String::from(value)}),
            "GWP" => Ok(MDRcurrency {code:String::from(value)}),
            "GYD" => Ok(MDRcurrency {code:String::from(value)}),
            "HKD" => Ok(MDRcurrency {code:String::from(value)}),
            "HNL" => Ok(MDRcurrency {code:String::from(value)}),
            "HRK" => Ok(MDRcurrency {code:String::from(value)}),
            "HTG" => Ok(MDRcurrency {code:String::from(value)}),
            "HUF" => Ok(MDRcurrency {code:String::from(value)}),
            "IDR" => Ok(MDRcurrency {code:String::from(value)}),
            "IEP" => Ok(MDRcurrency {code:String::from(value)}),
            "ILP" => Ok(MDRcurrency {code:String::from(value)}),
            "ILR" => Ok(MDRcurrency {code:String::from(value)}),
            "ILS" => Ok(MDRcurrency {code:String::from(value)}),
            "INR" => Ok(MDRcurrency {code:String::from(value)}),
            "IQD" => Ok(MDRcurrency {code:String::from(value)}),
            "IRR" => Ok(MDRcurrency {code:String::from(value)}),
            "ISJ" => Ok(MDRcurrency {code:String::from(value)}),
            "ISK" => Ok(MDRcurrency {code:String::from(value)}),
            "ITL" => Ok(MDRcurrency {code:String::from(value)}),
            "JMD" => Ok(MDRcurrency {code:String::from(value)}),
            "JOD" => Ok(MDRcurrency {code:String::from(value)}),
            "JPY" => Ok(MDRcurrency {code:String::from(value)}),
            "KES" => Ok(MDRcurrency {code:String::from(value)}),
            "KGS" => Ok(MDRcurrency {code:String::from(value)}),
            "KHR" => Ok(MDRcurrency {code:String::from(value)}),
            "KMF" => Ok(MDRcurrency {code:String::from(value)}),
            "KPW" => Ok(MDRcurrency {code:String::from(value)}),
            "KRW" => Ok(MDRcurrency {code:String::from(value)}),
            "KWD" => Ok(MDRcurrency {code:String::from(value)}),
            "KYD" => Ok(MDRcurrency {code:String::from(value)}),
            "KZT" => Ok(MDRcurrency {code:String::from(value)}),
            "LAJ" => Ok(MDRcurrency {code:String::from(value)}),
            "LAK" => Ok(MDRcurrency {code:String::from(value)}),
            "LBP" => Ok(MDRcurrency {code:String::from(value)}),
            "LKR" => Ok(MDRcurrency {code:String::from(value)}),
            "LRD" => Ok(MDRcurrency {code:String::from(value)}),
            "LSL" => Ok(MDRcurrency {code:String::from(value)}),
            "LTL" => Ok(MDRcurrency {code:String::from(value)}),
            "LUF" => Ok(MDRcurrency {code:String::from(value)}),
            "LVL" => Ok(MDRcurrency {code:String::from(value)}),
            "LYD" => Ok(MDRcurrency {code:String::from(value)}),
            "MAD" => Ok(MDRcurrency {code:String::from(value)}),
            "MDL" => Ok(MDRcurrency {code:String::from(value)}),
            "MGA" => Ok(MDRcurrency {code:String::from(value)}),
            "MGF" => Ok(MDRcurrency {code:String::from(value)}),
            "MKD" => Ok(MDRcurrency {code:String::from(value)}),
            "MLF" => Ok(MDRcurrency {code:String::from(value)}),
            "MMK" => Ok(MDRcurrency {code:String::from(value)}),
            "MNT" => Ok(MDRcurrency {code:String::from(value)}),
            "MOP" => Ok(MDRcurrency {code:String::from(value)}),
            "MRO" => Ok(MDRcurrency {code:String::from(value)}),
            "MRU" => Ok(MDRcurrency {code:String::from(value)}),
            "MTL" => Ok(MDRcurrency {code:String::from(value)}),
            "MTP" => Ok(MDRcurrency {code:String::from(value)}),
            "MUR" => Ok(MDRcurrency {code:String::from(value)}),
            "MVQ" => Ok(MDRcurrency {code:String::from(value)}),
            "MVR" => Ok(MDRcurrency {code:String::from(value)}),
            "MWK" => Ok(MDRcurrency {code:String::from(value)}),
            "MXN" => Ok(MDRcurrency {code:String::from(value)}),
            "MXP" => Ok(MDRcurrency {code:String::from(value)}),
            "MXV" => Ok(MDRcurrency {code:String::from(value)}),
            "MYR" => Ok(MDRcurrency {code:String::from(value)}),
            "MZM" => Ok(MDRcurrency {code:String::from(value)}),
            "MZN" => Ok(MDRcurrency {code:String::from(value)}),
            "NAD" => Ok(MDRcurrency {code:String::from(value)}),
            "NFD" => Ok(MDRcurrency {code:String::from(value)}),
            "NGN" => Ok(MDRcurrency {code:String::from(value)}),
            "NIO" => Ok(MDRcurrency {code:String::from(value)}),
            "NLG" => Ok(MDRcurrency {code:String::from(value)}),
            "NOK" => Ok(MDRcurrency {code:String::from(value)}),
            "NPR" => Ok(MDRcurrency {code:String::from(value)}),
            "NZD" => Ok(MDRcurrency {code:String::from(value)}),
            "OMR" => Ok(MDRcurrency {code:String::from(value)}),
            "PAB" => Ok(MDRcurrency {code:String::from(value)}),
            "PEH" => Ok(MDRcurrency {code:String::from(value)}),
            "PEI" => Ok(MDRcurrency {code:String::from(value)}),
            "PEN" => Ok(MDRcurrency {code:String::from(value)}),
            "PGK" => Ok(MDRcurrency {code:String::from(value)}),
            "PHP" => Ok(MDRcurrency {code:String::from(value)}),
            "PKR" => Ok(MDRcurrency {code:String::from(value)}),
            "PLN" => Ok(MDRcurrency {code:String::from(value)}),
            "PLZ" => Ok(MDRcurrency {code:String::from(value)}),
            "PTE" => Ok(MDRcurrency {code:String::from(value)}),
            "PYG" => Ok(MDRcurrency {code:String::from(value)}),
            "QAR" => Ok(MDRcurrency {code:String::from(value)}),
            "ROL" => Ok(MDRcurrency {code:String::from(value)}),
            "RON" => Ok(MDRcurrency {code:String::from(value)}),
            "RSD" => Ok(MDRcurrency {code:String::from(value)}),
            "RUB" => Ok(MDRcurrency {code:String::from(value)}),
            "RUR" => Ok(MDRcurrency {code:String::from(value)}),
            "RWF" => Ok(MDRcurrency {code:String::from(value)}),
            "SAR" => Ok(MDRcurrency {code:String::from(value)}),
            "SBD" => Ok(MDRcurrency {code:String::from(value)}),
            "SCR" => Ok(MDRcurrency {code:String::from(value)}),
            "SDD" => Ok(MDRcurrency {code:String::from(value)}),
            "SDG" => Ok(MDRcurrency {code:String::from(value)}),
            "SEK" => Ok(MDRcurrency {code:String::from(value)}),
            "SGD" => Ok(MDRcurrency {code:String::from(value)}),
            "SHP" => Ok(MDRcurrency {code:String::from(value)}),
            "SIT" => Ok(MDRcurrency {code:String::from(value)}),
            "SKK" => Ok(MDRcurrency {code:String::from(value)}),
            "SLL" => Ok(MDRcurrency {code:String::from(value)}),
            "SOS" => Ok(MDRcurrency {code:String::from(value)}),
            "SRD" => Ok(MDRcurrency {code:String::from(value)}),
            "SRG" => Ok(MDRcurrency {code:String::from(value)}),
            "SSP" => Ok(MDRcurrency {code:String::from(value)}),
            "STD" => Ok(MDRcurrency {code:String::from(value)}),
            "STN" => Ok(MDRcurrency {code:String::from(value)}),
            "SUR" => Ok(MDRcurrency {code:String::from(value)}),
            "SVC" => Ok(MDRcurrency {code:String::from(value)}),
            "SYP" => Ok(MDRcurrency {code:String::from(value)}),
            "SZL" => Ok(MDRcurrency {code:String::from(value)}),
            "THB" => Ok(MDRcurrency {code:String::from(value)}),
            "TJR" => Ok(MDRcurrency {code:String::from(value)}),
            "TJS" => Ok(MDRcurrency {code:String::from(value)}),
            "TMM" => Ok(MDRcurrency {code:String::from(value)}),
            "TMT" => Ok(MDRcurrency {code:String::from(value)}),
            "TND" => Ok(MDRcurrency {code:String::from(value)}),
            "TOP" => Ok(MDRcurrency {code:String::from(value)}),
            "TPE" => Ok(MDRcurrency {code:String::from(value)}),
            "TRL" => Ok(MDRcurrency {code:String::from(value)}),
            "TRY" => Ok(MDRcurrency {code:String::from(value)}),
            "TTD" => Ok(MDRcurrency {code:String::from(value)}),
            "TWD" => Ok(MDRcurrency {code:String::from(value)}),
            "TZS" => Ok(MDRcurrency {code:String::from(value)}),
            "UAH" => Ok(MDRcurrency {code:String::from(value)}),
            "UAK" => Ok(MDRcurrency {code:String::from(value)}),
            "UGS" => Ok(MDRcurrency {code:String::from(value)}),
            "UGX" => Ok(MDRcurrency {code:String::from(value)}),
            "USD" => Ok(MDRcurrency {code:String::from(value)}),
            "USN" => Ok(MDRcurrency {code:String::from(value)}),
            "USS" => Ok(MDRcurrency {code:String::from(value)}),
            "UYI" => Ok(MDRcurrency {code:String::from(value)}),
            "UYN" => Ok(MDRcurrency {code:String::from(value)}),
            "UYU" => Ok(MDRcurrency {code:String::from(value)}),
            "UYW" => Ok(MDRcurrency {code:String::from(value)}),
            "UZS" => Ok(MDRcurrency {code:String::from(value)}),
            "VEB" => Ok(MDRcurrency {code:String::from(value)}),
            "VEF" => Ok(MDRcurrency {code:String::from(value)}),
            "VES" => Ok(MDRcurrency {code:String::from(value)}),
            "VNC" => Ok(MDRcurrency {code:String::from(value)}),
            "VND" => Ok(MDRcurrency {code:String::from(value)}),
            "VUV" => Ok(MDRcurrency {code:String::from(value)}),
            "WST" => Ok(MDRcurrency {code:String::from(value)}),
            "XAF" => Ok(MDRcurrency {code:String::from(value)}),
            "XAG" => Ok(MDRcurrency {code:String::from(value)}),
            "XAU" => Ok(MDRcurrency {code:String::from(value)}),
            "XBA" => Ok(MDRcurrency {code:String::from(value)}),
            "XBB" => Ok(MDRcurrency {code:String::from(value)}),
            "XBC" => Ok(MDRcurrency {code:String::from(value)}),
            "XBD" => Ok(MDRcurrency {code:String::from(value)}),
            "XCD" => Ok(MDRcurrency {code:String::from(value)}),
            "XDR" => Ok(MDRcurrency {code:String::from(value)}),
            "XEU" => Ok(MDRcurrency {code:String::from(value)}),
            "XOF" => Ok(MDRcurrency {code:String::from(value)}),
            "XPD" => Ok(MDRcurrency {code:String::from(value)}),
            "XPF" => Ok(MDRcurrency {code:String::from(value)}),
            "XPT" => Ok(MDRcurrency {code:String::from(value)}),
            "XSU" => Ok(MDRcurrency {code:String::from(value)}),
            "XTS" => Ok(MDRcurrency {code:String::from(value)}),
            "XUA" => Ok(MDRcurrency {code:String::from(value)}),
            "XXX" => Ok(MDRcurrency {code:String::from(value)}),
            "YDD" => Ok(MDRcurrency {code:String::from(value)}),
            "YER" => Ok(MDRcurrency {code:String::from(value)}),
            "YUD" => Ok(MDRcurrency {code:String::from(value)}),
            "YUF" => Ok(MDRcurrency {code:String::from(value)}),
            "YUM" => Ok(MDRcurrency {code:String::from(value)}),
            "YUN" => Ok(MDRcurrency {code:String::from(value)}),
            "YUS" => Ok(MDRcurrency {code:String::from(value)}),
            "ZAR" => Ok(MDRcurrency {code:String::from(value)}),
            "ZMK" => Ok(MDRcurrency {code:String::from(value)}),
            "ZMW" => Ok(MDRcurrency {code:String::from(value)}),
            "ZRN" => Ok(MDRcurrency {code:String::from(value)}),
            "ZRZ" => Ok(MDRcurrency {code:String::from(value)}),
            "ZWC" => Ok(MDRcurrency {code:String::from(value)}),
            "ZWD" => Ok(MDRcurrency {code:String::from(value)}),
            "ZWL" => Ok(MDRcurrency {code:String::from(value)}),
            "ZWN" => Ok(MDRcurrency {code:String::from(value)}),
            "ZWR" => Ok(MDRcurrency {code:String::from(value)}),
            _ => Err(
                format!(
                    "Could not form an MDR standard currency code from string {}…",
                    value
                )
            )
        }
    }
}

/// The types of attachments that might come with a Europass credential.
enum EuropassAttachment {
    /// Portable Document Format
    PDF,
    /// Portable Network Graphics
    PNG,
    /// JPEG
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
