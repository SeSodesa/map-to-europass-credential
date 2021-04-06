/*!
This Rust library defines a mapping between SISU attainments and Europass credentials.
SISU attainments retrievable via GET requests are described in
the [SISU Swagger UI](https://sis-tuni.funidata.fi/ori/swagger-ui.html#/attainment-controller/getAttainmentsUsingGET),
whereas information concerning the Europass credentials can be found on Github,
under the [Europass Learning Model documentation](https://github.com/european-commission-europass/Europass-Learning-Model/tree/master/Credentials).

SISU provides the attainments in JSON format,
whereas the Europass API accepts XML documents as input.
The mapping therefore needs to perform this JSON → XML transformation.
The fields in the JSON provided by SISU also do not match the ones in Europass XML documents,
so in addition to a format transformation, a field transformation is performed as well.
*/

mod sisu_attainment;
mod europass_credential;
mod european_qualifications_framework;
mod national_qualification_frameworks;
