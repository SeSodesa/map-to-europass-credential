/*!
This Rust library defines a mapping between SISU attainments and Europass credentials.
SISU attainments retrievable via GET requests are described in
the [SISU Swagger UI](https://sis-tuni.funidata.fi/ori/swagger-ui.html#/attainment-controller/getAttainmentsUsingGET),
whereas information concerning the Europass credentials can be found on Github,
under the [Europass Learning Model documentation](https://github.com/european-commission-europass/Europass-Learning-Model/tree/master/Credentials).

SISU provides the attainments in JSON format, whereas the Europass API accepts XML documents as input.
The mapping therefore needs to perform this JSON → XML transformation.
The fields in the JSON provided by SISU also do not match the ones in Europass XML documents,
so in addition to a format transformation, a field transformation is performed as well.
*/

// These is to allow non-snake and camel case names, which are required to allow serde to deserialize
// (as in parse) SISU attainments automatically into the below SISUAttainment instances.
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use serde::{Serialize, Deserialize};
use serde_json;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
/// An attainment returned from the SISU database upon sending
/// a successful GET request to  the SISU Swagger API.
struct SISUAttainment {
    /// Employees who are responsible for giving the attainment
    acceptorPersons: Vec<PersonWithAttainmentAcceptorType>,
    /// Contains a set of strings localized to English, Finnish and Swedish.
    additionalInfo: LocalizedString,
    /// The official date this attainment was received.
    attainmentDate: String,
    /// Language of the attainment, typically one of the possible attainment languages of the assessment item
    attainmentLanguageUrn: String,
    /// Credit transfer information for an attainment that has been transferred.
    creditTransferInfo: CreditTransferInfo,
    /// The number of credits in this attainment.
    credits: serde_json::value::Number,
    /// A read only string of the document state.
    documentState: DocumentState,
    /// The date when this attainment will expire.
    /// Must conform to the date pattern.
    expiryDate: String,
    /// A result of grade average calculation.
    gradeAverage: GradeAverage,
    /// The index of the grade, within the grade scale, that represents the grade for this attainment
    gradeId: i32,
    /// The grade scale used in this attainment.
    /// Has to follow an OTM number pattern.
    gradeScaleId: String,
    /// A Unique identifier (a hash) for this attainment.
    /// Again, has to conform to the OTM number pattern.
    id: String,
    /// If true, this is a misregistration, replaced by a later attainment
    misregistration: bool,
    /// A justification for marking this attainment as misregistered.
    /// Length must be between 1--1024 characters.
    misregistrationRationale: String,
    /// Module content application which affects the same module as this module attainment is for.
    /// Must coform the the OTM-number pattern.
    moduleContentApplicationId: String,
    /// Organisations responsible for this attainment in various ways and fractions.
    /// Typically the same list as in the related CourseUnitRealisation
    organisations: Vec<OrganisationRoleShareBase>,
    /// The first names of the student.
    /// NOTE: Only for search purposes.
    personFirstNames: String,
    /// A private person identifier for the student who has this attainment.
    personId: String,
    /// The last name of the student.
    /// NOTE: Only for search purposes.
    personLastName: String,
    /// The student number of the student.
    /// NOTE: Only for search purposes.
    personStudentNumber: String,
    /// Indicates whether this is the primary attainment.
    /// There may be multiple attainments, for example if the student has tried to increase the grade.
    /// Primary attainment is not necessarily the latest attainment,
    /// as an earlier grade may be better than a later try.
    /// There can be only one primary attainment per student related to a module or a course unit cloud,
    /// or an attainment item.
    primary: bool,
    /// The date when this attainment was registered into the system.
    /// Must conform to the date pattern.
    registrationDate: String,
    /// The state of this attainment.
    state: AttainmentState,
    /// Student application from which this attainment is generated of.
    /// Must comply to OTM number pattern.
    studentApplicationId: String,
    /// Field of study related to this attainment.
    /// Musto comply with the URN pattern.
    studyFieldUrn: String,
    /// Study right to which this attainment is related to.
    /// Must be a valid OTM number.
    studyRightId: String,
    /// How many study weeks the credits of the attainment represents.
    /// This must be defined only for old attainments that used study weeks,
    /// in order to keep the original study week stored.
    studyWeeks: serde_json::value::Number,
    /// A type of attainment.
    /// One of AssessmentItemAttainment, CourseUnitAttainment, ModuleAttainment.
    attainmentType: AttainmentType,
    /// A public person identifier for the person who has done the verification action than converts assessment to attainment.
    verifierPersonId: String,
}

/// Describes a given person or textual personified role that has a given responsibility.
#[derive(Serialize, Deserialize)]
struct PersonWithAttainmentAcceptorType {
    /// The ID of the person, if available.
    /// Must be an OTM-compliant number.
    personId: String,
    /// The role of this person.
    /// Must comply to the URN pattern.
    roleUrn: String,
    /// Additional information related to this person.
    text: LocalizedString,
    /// The title of this person.
    title: LocalizedString,
}

/// A Role that an attainment creditor might possess.
#[derive(Serialize, Deserialize)]
enum RoleURN {
    ApprovedBy,
    CoordinatingSupervisor,
    CoordinatingProfessor,
    SupervisingProfessor,
    MoreSupervisingProfessor,
    Examinator,
    Supervisor,
    ThesisAdvisor,
    Examiner,
    PreliminaryExaminer,
    Opponent,
    Custos,
}

impl FromStr for RoleURN {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "urn:code:attainment-acceptor-type:approved-by" => Ok(Self::ApprovedBy),
            "urn:code:attainment-acceptor-type:coordinating-supervisor" => Ok(Self::CoordinatingSupervisor),
            "urn:code:attainment-acceptor-type:coordinating-professor" => Ok(Self::CoordinatingProfessor),
            "urn:code:attainment-acceptor-type:supervising-professor" => Ok(Self::SupervisingProfessor),
            "urn:code:attainment-acceptor-type:more-supervising-professor" => Ok(Self::MoreSupervisingProfessor),
            "urn:code:attainment-acceptor-type:examinator" => Ok(Self::Examinator),
            "urn:code:attainment-acceptor-type:supervisor" => Ok(Self::Supervisor),
            "urn:code:attainment-acceptor-type:thesis-advisor" => Ok(Self::ThesisAdvisor),
            "urn:code:attainment-acceptor-type:examiner" => Ok(Self::Examiner),
            "urn:code:attainment-acceptor-type:preliminary-examiner" => Ok(Self::PreliminaryExaminer),
            "urn:code:attainment-acceptor-type:opponent" => Ok(Self::Opponent),
            "urn:code:attainment-acceptor-type:custos" => Ok(Self::Custos),
            _   => Err(())
        }
    }
}

// A map <langCode, value> of plain strings containing localized versions of a text
#[derive(Serialize, Deserialize)]
struct LocalizedString {
    en: String,
    fi: String,
    sv: String,
}

/// Credit transfer information for an attainment that has been transferred.
#[derive(Serialize, Deserialize)]
struct CreditTransferInfo {
    /// The date of the credit transfer
    transferDate: String,
    /// Educational institution where this credit was originally attained.
    /// Must be URN code compliant.
    educationalInstitutionUrn: String,
    /// Specific international institution if educational institution refers to other/foreign institution.
    /// Must be a valid URN code.
    internationalInstitutionUrn: String,
    /// Description of the university or organisation if no suitable internationalInstitutionUrn
    /// can be given. Must be between 0--8000 characters long.
    organisation: String,
}

/// The state a document is in.
#[derive(Serialize, Deserialize)]
enum DocumentState {
    DRAFT,
    ACTIVE,
    DELETED
}

/// A type that contains information about an average grade calculation.
#[derive(Serialize, Deserialize)]
struct GradeAverage {
    /// The grade scale that was used.
    /// Must be a valid OTM number.
    gradeScaleId: String,
    /// The used calculation method.
    method: AverageCalculationMethod,
    /// The number of credits used in the calculation.
    totalIncludedCredits: serde_json::value::Number,
    /// Calculated average numerical grade.
    value: serde_json::value::Number
}

/// An enum describing how a grade average was calculated.
#[derive(Serialize, Deserialize)]
enum AverageCalculationMethod {
    COURSE_UNIT_ARITHMETIC_MEAN_WEIGHTING_BY_CREDITS,
    ARITHMETIC_MEAN_WEIGHTING_BY_CREDITS
}

/// Organisations responsible for an attainment in various ways and fractions.
#[derive(Serialize, Deserialize)]
struct OrganisationRoleShareBase {
    /// Identifier for an educational institution.
    /// Must conform to a URN pattern.
    educationalInstitutionUrn: String,
    /// The id of this organisation.
    /// Must conform to an OTM number pattern.
    organisationId: String,
    /// The role URN.
    /// Must conform to the URN pattern.
    roleUrn: String,
    /// The share, greater than zero and at most one.
    /// The shares of one role must sum to one.
    share: serde_json::value::Number,
}

/// A state an attainment could be in.
#[derive(Serialize, Deserialize)]
enum AttainmentState {
    ATTAINED,
    INCLUDED,
    SUBSTITUTED,
    FAILED
}

/// A type of attainment.
#[derive(Serialize, Deserialize)]
enum AttainmentType {
    AssessmentItemAttainment,
    CourseUnitAttainment,
    ModuleAttainment
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
