// initialization for password resets
// - add user to a bucket with reset code: Hashmap<user.uuid, code> (exp: 15min)
// - send user email with code