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
use std::str::FromStr;

/// An attainment returned from the SISU database upon sending
/// a successful GET request to  the SISU Swagger API.
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
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
    grade_id: serde_json::value::Number,
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
    #[serde(rename="type")]
    attainment_type: AttainmentType,
    /// A public person identifier for the person who has done the verification action than converts assessment to attainment.
    verifier_person_id: String,
}

/// Describes a given person or textual personified role that has a given responsibility.
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct PersonWithAttainmentAcceptorType {
    /// The ID of the person, if available.
    /// Must be an OTM-compliant number.
    person_id: String,
    /// The role of this person.
    /// Must comply to the URN pattern.
    role_urn: RoleURN,
    /// Additional information related to this person.
    text: LocalizedString,
    /// The title of this person.
    title: LocalizedString,
}

/// A Role that an attainment creditor might possess.
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

impl<'de> serde::Deserialize<'de> for RoleURN {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// A Visitor type required by custom serde parsers (deserializers)
        struct RoleURNVisitor;
        impl<'de> serde::de::Visitor<'de> for RoleURNVisitor {
            type Value = RoleURN;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or string representing a Foo")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<RoleURN, E> {
                match s {
                    "urn:code:attainment-acceptor-type:approved-by"
                        => Ok(RoleURN::ApprovedBy),
                    "urn:code:attainment-acceptor-type:coordinating-supervisor"
                        => Ok(RoleURN::CoordinatingSupervisor),
                    "urn:code:attainment-acceptor-type:coordinating-professor"
                        => Ok(RoleURN::CoordinatingProfessor),
                    "urn:code:attainment-acceptor-type:supervising-professor"
                        => Ok(RoleURN::SupervisingProfessor),
                    "urn:code:attainment-acceptor-type:more-supervising-professor"
                        => Ok(RoleURN::MoreSupervisingProfessor),
                    "urn:code:attainment-acceptor-type:examinator"
                        => Ok(RoleURN::Examinator),
                    "urn:code:attainment-acceptor-type:supervisor"
                        => Ok(RoleURN::Supervisor),
                    "urn:code:attainment-acceptor-type:thesis-advisor"
                        => Ok(RoleURN::ThesisAdvisor),
                    "urn:code:attainment-acceptor-type:examiner"
                        => Ok(RoleURN::Examiner),
                    "urn:code:attainment-acceptor-type:preliminary-examiner"
                        => Ok(RoleURN::PreliminaryExaminer),
                    "urn:code:attainment-acceptor-type:opponent"
                        => Ok(RoleURN::Opponent),
                    "urn:code:attainment-acceptor-type:custos"
                        => Ok(RoleURN::Custos),
                    _ => Err(E::invalid_value(serde::de::Unexpected::Str(s), &self)),
                }
            }
        }
        deserializer.deserialize_any(RoleURNVisitor)
    }
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
#[serde(rename_all="camelCase")]
struct CreditTransferInfo {
    /// The date of the credit transfer
    credit_transfer_date: String,
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
#[derive(Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
enum DocumentState {
    Draft,
    Active,
    Deleted,
}

/// A type that contains information about an average grade calculation.
#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
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
#[derive(Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
enum AverageCalculationMethod {
    CourseUnitArithmeticMeanWeightingByCredits,
    ArithmeticMeanWeightingByCredits
}

/// Organisations responsible for an attainment in various ways and fractions.
#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
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
#[derive(Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
enum AttainmentState {
    Attained,
    Included,
    Substituted,
    Failed,
}

/// A type of attainment.
#[derive(Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
enum AttainmentType {
    AssessmentItemAttainment,
    CourseUnitAttainment,
    ModuleAttainment
}

#[cfg(test)]
mod tests {
    /// Deconstrcuts an example response and tests that its fields match the expected contents
    #[test]
    fn parse_example_from_sisu_swagger_ui () {
        // Imports from the main module
        use crate::SISUAttainment;
        use crate::PersonWithAttainmentAcceptorType;
        use crate::RoleURN;
        use crate::LocalizedString;
        use crate::CreditTransferInfo;
        use crate::DocumentState;
        use crate::GradeAverage;
        use crate::OrganisationRoleShareBase;
        // Parse example JSON response
        let example_str = SISU_SWAGGER_UI_EXAMPLE_RESPONSE;
        let att_vec: Vec<SISUAttainment> = match serde_json::from_str(example_str) {
            Ok(att) => att,
            Err(e)  => panic!("{}", e)
        };
        let attainment = match att_vec.get(0) {
            Some(att) => att,
            None => panic!("No attainment in JSON array!")
        };
        // Test parsed SISU attainment.
        // Start by deconstructing the struct with Rust's pattern matching.
        let SISUAttainment {
            acceptor_persons,
            additional_info,
            attainment_date,
            attainment_language_urn,
            credit_transfer_info,
            credits,
            document_state,
            expiry_date,
            grade_average,
            grade_id,
            grade_scale_id,
            id,
            misregistration,
            misregistration_rationale,
            module_content_application_id,
            organisations,
            person_first_names,
            person_id,
            person_last_name,
            person_student_number,
            primary,
            registration_date,
            state,
            student_application_id,
            study_field_urn,
            study_right_id,
            study_weeks,
            attainment_type,
            verifier_person_id,
        } = &attainment;
        // Test acceptor_persons
        let only_acceptor = match acceptor_persons.get(0) {
            Some(acceptor) => acceptor,
            None => panic!("No acceptor found!")
        };
        // Deconstruct only acceptor
        let PersonWithAttainmentAcceptorType {
            person_id,
            role_urn,
            text,
            title,
        } = &only_acceptor;
        // Test for equality
        assert_eq!(person_id, "string");
        match role_urn {
            RoleURN::ApprovedBy => {}
            _ => panic!("Wrong acceptor URN type!")
        }
        // Test for LocalizedStrings in acceptor
        let LocalizedString {
            en,
            fi,
            sv
        } = &text;
        assert_eq!(en, "English version");
        assert_eq!(fi, "Finnish version");
        assert_eq!(sv, "Swedish version");
        let LocalizedString {
            en,
            fi,
            sv
        } = &title;
        assert_eq!(en, "English version");
        assert_eq!(fi, "Finnish version");
        assert_eq!(sv, "Swedish version");
        // Test additional_info
        assert_eq!(additional_info.en, "English version");
        assert_eq!(additional_info.fi, "Finnish version");
        assert_eq!(additional_info.sv, "Swedish version");
        // Test attainment_date
        assert_eq!(attainment_date, "string");
        // Test attainment_language_urn
        assert_eq!(attainment_language_urn, "string");
        // Test credit_transfer_info
        let CreditTransferInfo {
            credit_transfer_date,
            educational_institution_urn,
            international_institution_urn,
            organisation,
        } = &credit_transfer_info;
        assert_eq!(credit_transfer_date, "string");
        assert_eq!(educational_institution_urn, "string");
        assert_eq!(international_institution_urn, "string");
        assert_eq!(organisation, "string");
        // Test credits
        if let Some(credits) = credits.as_u64() {
            assert_eq!(credits, 0);
        }
        // Test document_state
        match document_state {
            DocumentState::Draft => {},
            _ => panic!("Incorrect document state!")
        }
        // Test expiry_date
        assert_eq!(expiry_date, "string");
        // Test grade_average
        let GradeAverage {
            grade_scale_id,
            method,
            total_included_credits,
            value,
        } = &grade_average;
        assert_eq!(grade_scale_id, "string");
        match method {
            crate::AverageCalculationMethod::CourseUnitArithmeticMeanWeightingByCredits
                => {},
            crate::AverageCalculationMethod::ArithmeticMeanWeightingByCredits
                => panic!("Wrong average calculation method!")
        }
        if let Some(credits) = total_included_credits.as_u64() {
            assert_eq!(credits, 0);
        } else {
            panic!("No total credits!")
        }
        if let Some(credits) = value.as_u64() {
            assert_eq!(credits, 0);
        } else {
            panic!("No total credits!")
        }
        // Test grade_id
        if let Some(id) = grade_id.as_i64() {
            assert_eq!(id, 0);
        } else {
            panic!("invalid grade id!")
        }
        // Test grade_scale_id
        assert_eq!(grade_scale_id, "string");
        // Test id
        assert_eq!(id, "string");
        // Test misregistration
        assert!(misregistration);
        // Test misregistration_rationale
        assert_eq!(misregistration_rationale, "string");
        // Test module_content_application_id
        assert_eq!(module_content_application_id, "string");
        // Test organisations
        let only_organisation = match organisations.get(0) {
            Some(org)   => org,
            None        => panic!("No organisations!")
        };
        let OrganisationRoleShareBase {
            educational_institution_urn,
            organisation_id,
            role_urn,
            share,
        } = &only_organisation;
        assert_eq!(educational_institution_urn, "string");
        assert_eq!(organisation_id, "string");
        assert_eq!(role_urn, "string");
        if let Some(number) = share.as_f64() {
            assert_eq!(number, 0.0);
        } else {
            panic!("No valid organisation share ∊ [0,1]!")
        }
        // Test person_first_names
        assert_eq!(person_first_names, "string");
        // Test person_id
        assert_eq!(person_id, "string");
        // Test person_last_name
        assert_eq!(person_last_name, "string");
        // Test person_student_number
        assert_eq!(person_student_number, "string");
        // Test primary
        assert!(primary);
        // Test registration_date
        assert_eq!(registration_date, "string");
        // Test state
        match state {
            crate::AttainmentState::Attained => {},
            _ => panic!("Wrong attainment state!")
        }
        // Test student_application_id
        assert_eq!(student_application_id, "string");
        // Test study_field_urn
        assert_eq!(study_field_urn, "string");
        // Test study_right_id
        assert_eq!(study_right_id, "string");
        // Test study_weeks
        if let Some(number) = study_weeks.as_u64() {
            assert_eq!(number, 0);
        } else {
            panic!("Invalid study week format!")
        }
        // Test attainment_type
        match attainment_type {
            crate::AttainmentType::AssessmentItemAttainment => {},
            _ => panic!("Wrong attainment type!")
        }
        // Test verifier_person_id
        assert_eq!(verifier_person_id, "string");
    }
    /// The example JSON response found in the SISU Swagger UI:
    /// https://sis-tuni.funidata.fi/ori/swagger-ui.html#/attainment-controller/getAttainmentsUsingGET
    const SISU_SWAGGER_UI_EXAMPLE_RESPONSE: &'static str =
        r#"[
              {
                "acceptorPersons": [
                  {
                    "personId": "string",
                    "roleUrn": "urn:code:attainment-acceptor-type:approved-by",
                    "text": {
                      "en": "English version",
                      "fi": "Finnish version",
                      "sv": "Swedish version"
                    },
                    "title": {
                      "en": "English version",
                      "fi": "Finnish version",
                      "sv": "Swedish version"
                    }
                  }
                ],
                "additionalInfo": {
                  "en": "English version",
                  "fi": "Finnish version",
                  "sv": "Swedish version"
                },
                "attainmentDate": "string",
                "attainmentLanguageUrn": "string",
                "creditTransferInfo": {
                  "creditTransferDate": "string",
                  "educationalInstitutionUrn": "string",
                  "internationalInstitutionUrn": "string",
                  "organisation": "string"
                },
                "credits": 0,
                "documentState": "DRAFT",
                "expiryDate": "string",
                "gradeAverage": {
                  "gradeScaleId": "string",
                  "method": "COURSE_UNIT_ARITHMETIC_MEAN_WEIGHTING_BY_CREDITS",
                  "totalIncludedCredits": 0,
                  "value": 0
                },
                "gradeId": 0,
                "gradeScaleId": "string",
                "id": "string",
                "misregistration": true,
                "misregistrationRationale": "string",
                "moduleContentApplicationId": "string",
                "organisations": [
                  {
                    "educationalInstitutionUrn": "string",
                    "organisationId": "string",
                    "roleUrn": "string",
                    "share": 0
                  }
                ],
                "personFirstNames": "string",
                "personId": "string",
                "personLastName": "string",
                "personStudentNumber": "string",
                "primary": true,
                "registrationDate": "string",
                "state": "ATTAINED",
                "studentApplicationId": "string",
                "studyFieldUrn": "string",
                "studyRightId": "string",
                "studyWeeks": 0,
                "type": "AssessmentItemAttainment",
                "verifierPersonId": "string"
              }
        ]"#;
}
