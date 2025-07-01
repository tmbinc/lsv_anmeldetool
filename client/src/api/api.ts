import { Apity, type ApiResponse } from '@cocreators-ee/apity';
import type { components, paths } from './api.d';

export const apity = Apity.for<paths>();

const forceEnvironment: 'development' | 'production-test' | 'production' | undefined = undefined;

export function getServerUrl(): string {
	const env = forceEnvironment || import.meta.env.VITE_ENV;
	if (env === 'development') {
		return 'http://localhost:5173';
	} else if (env === 'production-test') {
		return 'https://beta.reg.lsv1873.de';
	} else {
		return 'https://anmeldung.lsv1873.de';
	}
}

// global configuration
apity.configure({
    baseUrl: getServerUrl(),
    init: {
      mode: 'cors'
      // headers: {}
    },
  })

export const listOrgs = apity.path('/api/v1/orgs').method('get').create();
export const getOrg = apity.path('/api/v1/org/{org}').method('get').create();
export const updateOrg = apity.path('/api/v1/org/{org}').method('put').create();
export const addOrg = apity.path('/api/v1/org').method('post').create();
export const listEvents = apity.path('/api/v1/events').method('get').create();
export const getEvent = apity.path('/api/v1/event/{event}').method('get').create();
export const updateEvent = apity.path('/api/v1/event/{event}').method('put').create();
export const addEvent = apity.path('/api/v1/event').method('post').create();
export const getEventOrgs = apity.path('/api/v1/event/{event}/orgs').method('get').create();
export const setEventOrgState = apity.path('/api/v1/event/{event}/org/{org}').method('put').create();
export const getOrgEvents = apity.path('/api/v1/org/{org}/events').method('get').create();

export const getTeamsForOrgEvent = apity.path('/api/v1/org/{org}/{event}/teams').method('get').create();
export const createTeam = apity.path('/api/v1/team').method('post').create();
export const updateTeam = apity.path('/api/v1/team').method('put').create();
export const deleteTeam = apity.path('/api/v1/team/{team}').method('delete').create();
export const getTeam = apity.path('/api/v1/team/{team}').method('get').create();
export const setTeamPresent = apity.path('/api/v1/team/{team}/present').method('put').create();

export const getGroupsForEvent = apity.path('/api/v1/event/{event}/groups').method('get').create();
export const getGroup = apity.path('/api/v1/group/{group}').method('get').create();
export const deleteGroup = apity.path('/api/v1/group/{group}').method('delete').create();
export const updateGroup = apity.path('/api/v1/group').method('put').create();
export const createGroup = apity.path('/api/v1/event/{event}/groups').method('post').create();

export const createOrgSelfReg = apity.path("/api/v1/orgs/self_register").method('post').create();
export const getOrgEventStatus = apity.path("/api/v1/org/{org}/{event}").method('get').create();

export const authLogin = apity.path('/api/v1/auth').method('post').create();
export const whoAmI = apity.path('/api/v1/auth').method('get').create();
export const authLogout = apity.path('/api/v1/auth').method('delete').create();
export const authOrgLogin = apity.path('/api/v1/org/{org}/auth').method('post').create();

export const inviteOrgToEvent = apity.path('/api/v1/org/{org}/{event}/invite').method("post").create();
export const listInvites = apity.path('/api/v1/invites').method('get').create();
export const sendInvite = apity.path('/api/v1/invites').method('post').create();

export const createQuestionnaire = apity.path('/api/v1/event/{event}/questionnaire').method('post').create();
export const getQuestionnaireForEvent = apity.path('/api/v1/event/{event}/questionnaire').method('get').create();
export const updateQuestionnaire = apity.path('/api/v1/questionnaire').method('put').create();
export const deleteQuestionnaire = apity.path('/api/v1/questionnaire/{id}').method('delete').create();
export const getQuestionnaireForOrgEvent = apity.path('/api/v1/org/{org}/{event}/questionnaire').method('get').create();

export const getQuestionnaireAnswerForOrgEvent = apity.path('/api/v1/org/{org}/{event}/questionnaire_answer').method('get').create();
export const updateQuestionnaireAnswer = apity.path('/api/v1/questionnaire_answer').method('put').create();
export const getQuestionnaireAnswersForEvent = apity.path('/api/v1/event/{event}/questionnaire_answers').method('get').create();

export const setPairings = apity.path('/api/v1/event/{event}/pairings/{group}/{round}').method('put').create();
export const getPairings = apity.path('/api/v1/event/{event}/pairings').method('get').create();

export const setRooms = apity.path('/api/v1/event/{event}/rooms/{group}').method('put').create();
export const getRooms = apity.path('/api/v1/event/{event}/rooms').method('get').create();

export const getResults = apity.path('/api/v1/event/{event}/results').method('get').create();
export const setResults = apity.path('/api/v1/event/{event}/results/{group}/{round}').method('put').create();

export const getTimetable = apity.path('/api/v1/event/{event}/timetable').method('get').create();
export const setTimetable = apity.path('/api/v1/event/{event}/timetable').method('put').create();

export type Invite = components["schemas"]["Invite"];
export type Org = components["schemas"]["Org"];
export type Event = components["schemas"]["Event"];
export type EventOrg = components["schemas"]["EventOrg"];
export type OrgEvent = components["schemas"]["OrgEvent"];
export type Team = components["schemas"]["Team"];
export type NewTeam = components["schemas"]["NewTeam"];
export type Group = components["schemas"]["Group"];
export type EventOrgState = components["schemas"]["EventOrgState"];
export type Questionnaire = components["schemas"]["Questionnaire"];
export type QuestionnaireAnswer = components["schemas"]["QuestionnaireAnswer"];
export type Pairing = components["schemas"]["Pairing"];
export type PairingForEvent = components["schemas"]["PairingForEvent"];
export type PairingEntry = components["schemas"]["PairingEntry"];
export type Room = components["schemas"]["Room"];
export type EventQuestionnaireAnswers = components["schemas"]["EventQuestionnaireAnswers"];
export type ResultEntry = components["schemas"]["ResultEntry"];
export type ResultsForEvent = components["schemas"]["ResultsForEvent"];
export type TeamResult = components['schemas']["TeamResult"];
export type TimetableForEvent = components["schemas"]["TimetableForEvent"];
export type TimetableRow = components["schemas"]["TimetableRow"];
export type TimetableEntry = components["schemas"]["TimetableEntry"];
