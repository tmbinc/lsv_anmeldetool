DELETE FROM users;
INSERT INTO users (email, hash, created_at) VALUES ("Felix@Dom.ke", "$argon2i$v=19$m=4096,t=3,p=1$c3VwZXJzZWN1cmVzYWx0$bHI/Hp6fr0a6nAHEzFB+8RVJL5Dh48yMmrQv+Wv6+UQ", DateTime('now'));
UPDATE orgs SET contact_email = "Felix@Dom.ke";
UPDATE org_secrets SET secret="secret";
