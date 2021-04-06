/*!
This submodule defines the Rust representation of the controlled vocabularies
published in the Named Authority Lists of Europass. These refer to different
types of classifications of the objects defined in the Europass Learning Model.

See https://europa.eu/europass/en/europass-digital-credentials-interoperability.
*/

/// This is an enumeration of the standard list of EU accreditation types.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/accreditation/25831c2.
pub enum AccreditationType {
    /// A licencing procedure applied at the level of an organisation.
    /// Institutional Licencing implies permission for the institution to operate,
    /// and is awarded by Public Authorities or delegates thereof.
    InstitutionalLicense,
    /// A quality assurance procedure applied at the level of one or several programmes.
    /// Programme Quality Assurance leads to a QA Decision,
    /// but does not have any legal implications.
    /// Programme Quality Assurance may be given within the context of private QA labels.
    ProgramQualityAssurance,
    /// A quality assurance procedure applied at the level of an organisation.
    /// Institutional Quality Assurance leads to a QA Decision,
    /// but does not have any legal implications.
    /// Institutional Quality Assurance may be provided within the context of
    /// private QA labels.
    InstitutionalQualityAssurance,
    /// A licencing procedure applied at the level of one or several programmes.
    /// Institutional Licencing implies permission for an institution to provide
    /// a specific programme, and is awarded by Public Authorities or delegates thereof.
    ProgramLicense,
}

/// Europass standard list of assessment types.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/assessment/25831c2
pub enum AssessmentType {
    PeerAssessment,
    MarkedAssignment,
    ContinuousEvaluation,
    Portfolio,
    GroupPerformance,
    PracticalAssessment,
    WrittenExamination,
    LevelOfAttendance,
    ProjectWork,
    PeerReview,
    Quiz,
    ProblemBasedLearning,
    OralExamination,
    ArtefactAssessment,
}

/// The Europass Standard List of Communication Channel Types provides a list of online information transmission media categories.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/com-channel/25831c2.
pub enum CommunicationChannelType {
    Post,
    Email,
    MobilePhone,
    Fax,
    Web,
}

/// The Europass Standard List of Communication Channel Usage Types provides a list of descriptors of information transmission settings.
/// See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/com-channel-usg.
pub enum CommunicationChannelUsageType {
    Personal,
    Legal,
    Business,
    Mobile,
}

/// Europass Vocabularies consist of a set of simplified, re-usable and extensible data models that capture the fundamental characteristics entities.
/// They are primarily used in the Europass CV template, Europass Digital Credentials and Europass Learning Opportunities. Any entity wishing to be
/// interoperable with these services can benefit by using the models.
///
/// See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/encoding
pub enum ContentEncodingTypes {
    Base64,
}

///  The Europass Standard List of Educational Credit Systems distinguishes between existing and widely used European credit systems, e.g. for higher education (ECTS) and for vocational education and training (ECVET).
///  See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/education-credit.
pub enum EducationalCreditSystems {
    /// European credit system for vocational education and training
    VocationalSystem,
    /// European Credit Transfer System
    CreditTransferSystem,
}

/// An enumeration of the standard Europass credential types.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/credential/25831c2.
pub enum CredentialType {
    /// Represents a credential which describes that an activity has been or is being done
    LearningActivity,
    /// Represents the award of a qualification by an institution that is accredited to do so, confirmed by its inclusion in the Europass Accreditation Database.
    QualificationAward,
    /// Represents the award of a higher education qualification by an institution that is accredited to do so, confirmed by its inclusion in the Europass Accreditation Database, and including all diploma supplement information.
    DiplomaSupplement,
    /// Represents a credential which describes that the user has received a right.
    LearningEntitlement,
    /// This is the default Europass credential type.
    Generic,
}

/// An enumeration of standard Europass entitlement statuses.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/entitlement-status/25831c2
pub enum EntitlementStatus {
    /// A prospective entitlement awards the right to apply for a (specific or class of)
    /// learning opportunity, employment or membership.
    Prospective,
    /// An actual entitlement grants the right to pursue a learning opportunity,
    /// employment or membership automatically.
    Actual,
}

/// An enumeration of standard Europass entitlement types.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/entitlement/25831c2
pub enum EntitlementType {
    Occupation,
    LearningOpportunity,
    Membership
}

/// The Europass Standard List of Learning Activity Types categorises and labels activities that learners can engage with in order to achieve pre-defined learning outcomes.
/// See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/learning-activity.
pub enum LearningActivityType {
    /// Lab / simulation / practice coursework
    PracticalCoursework,
    JobExperience,
    Volunteering,
    Research,
    SelfMotivatedStudy,
    ELearningCoursework,
    Internship,
    Apprenticeship,
    /// Workshop, seminar or conference
    Workshop,
    EducationalProgramme,
    ClassroomCoursework,
}

/// The Europass Standard List of Learning Opportunity Types provides an array of potential delivery formats of organised learning.
/// See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/learning-opportunity
pub enum LearningOpportunityType {
    Course,
    ProgrammeModule,
    Mentoring,
    MOOC,
    Apprenticeship,
    StudyVisit,
    ShortLearningProgramme,
    Internship,
    EducationalProgramme,
    Class,
    ServiceLearning,
    Thesis,
}

///  The Europass Standard List of Learning Schedule Types allows, in a standardised way, the indication of the intensity of learning, from light part time to full time engagement.
///  See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/learning-schedule/25831c2
pub enum LearningScheduleType {
    /// Part time light (less than 8 hours)
    PartTimeLight,
    /// Full time (more then 30 hours)
    FullTime,
    /// Part time intensive (8 to 30 hours)
    PartTimeIntensive,
}

/// The Europass Standard List of Learning Setting Types provides terms to distinguish between formal learning taking place in a more organised and structured environment,
/// and non-formal learning that is more flexible and self-paced, however still involves some form of learning support.
///
/// See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/learning-setting
pub enum LearningSettingType {
    FormalLearning,
    NonFormalLearning,
}

/// This refers to the `VerificationType` in the documentation.
/// Therefore this type is provided simply as an indirect means of accessing
/// an instance of `VerificationType`.
pub struct SupervisionAndVerificationType (VerificationType);

///  The Europass Standard List of Modes of Learning and Assessment provides a list of distinct means by which learning and assessment can be carried out.
///  See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/learning-assessment
pub enum ModeOfLearningType {
    WorkBased,
    ProjectBased,
    Presential,
    Online,
    Blended,
    ResearchLabBased,
}

/// The Europass Standard List of Target Groups provides a custom vocabulary to describe groups of learners that a learning opportunity, and corresponding credential, is tailored and/or best suited for.
/// See https://op.europa.eu/en/web/eu-vocabularies/dataset/-/resource?uri=http://publications.europa.eu/resource/dataset/target-group
pub enum LearningTargetGroup {
    HighAchievers,
    NonNativeSpeakers,
    RequiringEmploymentRetraining,
    InTertiaryEducationEQF6,
    CompletedPrimaryEducation,
    InCompulsoryEducation,
    CompletedTertiaryEducationEQF7,
    InPrimaryEducation,
    WorkedLessThan3Years,
    CompletedTertiaryEducationEQF8,
    CompletedCompulsoryEducation,
    InTertiaryEducationEQF7,
    InTertiaryEducationEQF8,
    Migrants,
    Worked3to10Years,
    WorkedOver10Years,
    WithLearningDisability,
    NativeSpeakers,
    CompletedTertiaryEducationEQF6,
    LowAchievers,
}

/// An enumeration of the standard Europass verification statuses.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/verification-status/25831c2.
pub enum VerificationStatus {
    Gray,
    Green,
    Red,
}

/// An enumeration of the standard Europass verification statuses.
/// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/verification/25831c2.
pub enum VerificationType {
    /// Check whether the the person presenting the credential is the owner.
    /// Includes:
    ///
    /// (a) Comparison of the Name and Date of Birth written in
    ///     the credential with the name and date of birth stored
    ///     in a person's national eID.
    Owner,
    /// Check whether the the credential has been revoked.
    /// Includes:
    /// (a) check if revocation certificate has been published to address indicated in
    ///     the credential.
    /// (b) Check if revocation certificate has been published to national revocation list
    /// (c) Check if revocation certificate has been published to EU revocation list
    Revocation,
    /// Check as to whether the credential has been issued according to
    /// the standard specified for that credential-type. Includes:
    ///
    /// (a) technical check of the XML file for validity
    /// (b) check if credential has basic required fields for all credentials
    /// (c) check if credential has followed the rules indicated against
    ///     the application profile for that specific credential-type stored at
    ///     data.europa.eu
    Format,
    /// Check whether the credential is still valid. Includes:
    ///
    /// (a) check against expiry information contained with credential
    Validity,
    /// Custom check defined by a third-party credential verifier
    Custom,
    /// Check whether the awarding body is authorised to issue the credential.
    /// Includes:
    ///
    /// (a) check if credential is an NQF qualification award.
    /// (b) If it is, check that institution or qualification is in
    ///     the accreditation database by comparing UID of qualification
    ///     and/or institution with that in the accreditation database
    Accreditation,
    /// Check as to whether the credential has been tampered with. Includes:
    ///
    /// (a) check that the eSeal is valid
    Seal,
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
pub enum MDRunit {
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

/// The Currency authority table is a controlled vocabulary that lists concepts associated with currencies and currency subunits.
/// The concepts included are correlated with the ISO 4217 international standard.
/// The Currency authority table is updated based on the stakeholders’ needs.
/// Contributions to it are accepted following a review made by EU Vocabularies team of the Publications Office of the EU.
/// The Currency authority table is maintained by the Publications Office of the European Union on the EU Vocabularies website.
/// See http://publications.europa.eu/resource/authority/currency.
///
/// Can only be constructed from a known MDR currency code.
pub struct MDRcurrency {
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
