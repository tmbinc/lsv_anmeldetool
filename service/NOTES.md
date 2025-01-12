
team
- id
- event
- org

org
- id
- name
- org contact phone
- org contact email

org_event
org.id | event.id

player
- id
- name

player_team
player.id | team.id

event
- id
- name

group:
- id
- name

group x event
group.id | event.id


"[ADMIN] Create an org"
"[ADMIN] Read org"
"[ADMIN] List orgs"
"[ADMIN] Update org"
"[ADMIN] Delete org"

"[ADMIN] Create an event"
"[ADMIN] Read event"
"[ADMIN] List events"
"[ADMIN] Update event"
"[ADMIN] Delete event"

"[ADMIN] Add org to event"
"[ADMIN] Delete org from event"

"[ORG] create team for event"
"[ORG] edit team for event"
"[ORG] delete team for event"

"[ORG] set group for team"
