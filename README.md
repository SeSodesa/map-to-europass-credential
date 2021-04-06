# ↦ Europass Credential

This Rust library defines a mapping between University-specific accreditations
and Europass credentials. The Europass API accepts
[XML documents](https://github.com/european-commission-europass/Europass-Learning-Model/tree/master/Credentials) as input.
The mapping therefore needs to perform a data → XML transformation.
The fields of University-specific accreditations are also
unlikely to have a one-to-one mapping between the
[Europass Credential XML Model](https://github.com/european-commission-europass/Europass-Learning-Model/blob/master/Credentials/Credentials_Learning_Model.md)
fields, so this transformation needs to be performed as well.

## Implemented conversions (or conversions being worked on)

This section lists the conversions that have been implemented
and some details related to each conversion.

### Tampere University

SISU attainments retrievable via GET requests are described in
the [SISU Swagger UI](https://sis-tuni.funidata.fi/ori/swagger-ui.html#/attainment-controller/getAttainmentsUsingGET).
SISU provides the attainments in JSON format.
