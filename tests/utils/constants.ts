const decimals = 9;
const bumpRangeInclusive = [0, 255];

const seeds = {
    borrowRate: "borrow_rate",
    platformConfig: "platform_config",
    market: "market",
    vault: "vault",
    price: "price",
};

const errors = {
    noDefaultPubkey: "No default pubkey",
    maxLltvExceeded: "Max lltv exceeded",
    maxFeeExceeded: "Max fee exceeded",
    marketNotCreated: "Market not created",
    inconsistentInput: "Inconsistent input",

    valueZero: "Value zero",

    addressConstraintViolated: "An address constraint was violated",
};

export { decimals, bumpRangeInclusive, seeds, errors };
