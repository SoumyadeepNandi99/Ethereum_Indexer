use diesel::prelude::*;
use crate::schema::validators;

#[derive(Debug, Serialize, Deserialize, Queryable)]
struct Validator {
    id: i32,
    public_key: String,
    missed_attestations: i32,
}

// Function to get the entire network's participation rate
pub fn get_network_participation_rate(conn: &PgConnection) -> QueryResult<f64> {
    use diesel::dsl::count;

    let validators_count = validators::table
        .select(count(validators::id))
        .first::<i64>(conn)?;

    let total_missed_attestations: i64 = validators::table
        .select(diesel::dsl::sum(validators::missed_attestations))
        .first(conn)?;

    let epochs = 5; // Number of recent epochs to consider
    let slots_per_epoch = 32; // Number of slots per epoch
    let validator_set_size = 1024; // Number of validators in the set

    let total_slots = epochs * slots_per_epoch * validator_set_size;
    let participation_rate = 1.0 - (total_missed_attestations as f64) / (total_slots as f64);

    Ok(participation_rate)
}

// Function to get a specific validator's participation rate
pub fn get_validator_participation_rate(conn: &PgConnection, validator_id: i32) -> QueryResult<f64> {
    use diesel::dsl::count;

    let validator = validators::table
        .filter(validators::id.eq(validator_id))
        .first::<Validator>(conn)?;

    let epochs = 5; // Number of recent epochs to consider
    let slots_per_epoch = 32; // Number of slots per epoch

    let total_slots = epochs * slots_per_epoch;
    let participation_rate = 1.0 - (validator.missed_attestations as f64) / (total_slots as f64);

    Ok(participation_rate)
}
