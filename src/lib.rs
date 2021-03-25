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

use serde::{Serialize, Deserialize};
use serde_json;

/// An attainment returned from the SISU database upon sending
/// a successful GET request to  the SISU Swagger API.
struct SISUAttainment {
    /// Employees who are responsible for giving the attainment
    acceptor_persons: Vec<PersonWithAttainmentAcceptorType>,
    /// Contains a set of strings localized to English, Finnish and Swedish.
    additional_info: LocalizedString,
    /// The official date this attainment was received.
    attainment_date: String,
    /// Language of the attainment, typically one of the possible attainment languages of the assessment item
    attainment_language_urn: String,
    /// Credit transfer information for an attainment that has been transferred.
    credit_transfer_info: CreditTransferInfo,
    /// The number of credits in this attainment.
    credits: serde_json::value::Number,
    /// A read only string of the document state.
    document_state: DocumentState,
    /// The date when this attainment will expire.
    /// Must conform to the date pattern.
    expiry_date: String,
    /// A result of grade average calculation.
    grade_average: GradeAverage,
    /// The index of the grade, within the grade scale, that represents the grade for this attainment
    grade_id: i32,
    /// The grade scale used in this attainment.
    /// Has to follow an OTM number pattern.
    grade_scale_id: String,
    /// A Unique identifier (a hash) for this attainment.
    /// Again, has to conform to the OTM number pattern.
    id: String,
    /// If true, this is a misregistration, replaced by a later attainment
    misregistration: bool,
    /// A justification for marking this attainment as misregistered.
    /// Length must be between 1--1024 characters.
    misregistration_rationale: String,
    /// Module content application which affects the same module as this module attainment is for.
    /// Must coform the the OTM-number pattern.
    module_content_application_id: String,
    /// Organisations responsible for this attainment in various ways and fractions.
    /// Typically the same list as in the related CourseUnitRealisation
    organisations: Vec<OrganisationRoleShareBase>,
    /// The first names of the student.
    /// NOTE: Only for search purposes.
    person_first_names: String,
    /// A private person identifier for the student who has this attainment.
    person_id: String,
    /// The last name of the student.
    /// NOTE: Only for search purposes.
    person_last_name: String,
    /// The student number of the student.
    /// NOTE: Only for search purposes.
    person_student_number: String,
    /// Indicates whether this is the primary attainment.
    /// There may be multiple attainments, for example if the student has tried to increase the grade.
    /// Primary attainment is not necessarily the latest attainment,
    /// as an earlier grade may be better than a later try.
    /// There can be only one primary attainment per student related to a module or a course unit cloud,
    /// or an attainment item.
    primary: bool,
    /// The date when this attainment was registered into the system.
    /// Must conform to the date pattern.
    registration_date: String,
    /// The state of this attainment.
    state: AttainmentState,
    /// Student application from which this attainment is generated of.
    /// Must comply to OTM number pattern.
    student_application_id: String,
    /// Field of study related to this attainment.
    /// Musto comply with the URN pattern.
    study_field_urn: String,
    /// Study right to which this attainment is related to.
    /// Must be a valid OTM number.
    study_right_id: String,
    /// How many study weeks the credits of the attainment represents.
    /// This must be defined only for old attainments that used study weeks,
    /// in order to keep the original study week stored.
    study_weeks: serde_json::value::Number,
    /// A type of attainment.
    /// One of AssessmentItemAttainment, CourseUnitAttainment, ModuleAttainment.
    attainment_type: AttainmentType,
    /// A public person identifier for the person who has done the verification action than converts assessment to attainment.
    verifier_person_id: String,
}

/// Describes a given person or textual personified role that has a given responsibility.
struct PersonWithAttainmentAcceptorType {
    /// The ID of the person, if available.
    /// Must be an OTM-compliant number.
    person_id: String,
    /// The role of this person.
    /// Must comply to the URN pattern.
    role_urn: String,
    /// Additional information related to this person.
    text: LocalizedString,
    /// The title of this person.
    title: LocalizedString,
}

// A map <langCode, value> of plain strings containing localized versions of a text
struct LocalizedString {
    en: String,
    fi: String,
    sv: String,
}

/// Credit transfer information for an attainment that has been transferred.
struct CreditTransferInfo {
    /// The date of the credit transfer
    transfer_date: String,
    /// Educational institution where this credit was originally attained.
    /// Must be URN code compliant.
    educational_institution_urn: String,
    /// Specific international institution if educational institution refers to other/foreign institution.
    /// Must be a valid URN code.
    international_institution_urn: String,
    /// Description of the university or organisation if no suitable internationalInstitutionUrn
    /// can be given. Must be between 0--8000 characters long.
    organisation: String,
}

/// The state a document is in.
enum DocumentState {
    Draft,
    Active,
    Deleted,
}

/// A type that contains information about an average grade calculation.
struct GradeAverage {
    /// The grade scale that was used.
    /// Must be a valid OTM number.
    grade_scale_id: String,
    /// The used calculation method.
    method: AverageCalculationMethod,
    /// The number of credits used in the calculation.
    total_included_credits: serde_json::value::Number,
    /// Calculated average numerical grade.
    value: serde_json::value::Number
}

/// An enum describing how a grade average was calculated.
enum AverageCalculationMethod {
    CourseUnitArithmeticMeanWeightingByCredits,
    ArithmeticMeanWeightingByCredits
}

/// Organisations responsible for an attainment in various ways and fractions.
struct OrganisationRoleShareBase {
    /// Identifier for an educational institution.
    /// Must conform to a URN pattern.
    educational_institution_urn: String,
    /// The id of this organisation.
    /// Must conform to an OTM number pattern.
    organisation_id: String,
    /// The role URN.
    /// Must conform to the URN pattern.
    role_urn: String,
    /// The share, greater than zero and at most one.
    /// The shares of one role must sum to one.
    share: serde_json::value::Number,
}

/// A state an attainment could be in.
enum AttainmentState {
    Attained,
    Included,
    Substituted,
    Failed
}

/// A type of attainment.
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
