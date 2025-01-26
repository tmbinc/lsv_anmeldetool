import { Apity, type ApiResponse } from '@cocreators-ee/apity';
import type { components, paths } from './api.d';

export const apity = Apity.for<paths>();

const forceEnvironment: 'development' | 'production-test' | 'production' | undefined = 'development';

export function getServerUrl(): string {
	const env = forceEnvironment || import.meta.env.VITE_ENV;
	if (env === 'development') {
		return 'http://localhost:8080';
	} else if (env === 'production-test') {
		return 'https://beta.reg.lsv1873.de';
	} else {
		return 'https://reg.lsv1873.de';
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

export const getGroupsForEvent = apity.path('/api/v1/event/{event}/groups').method('get').create();
export const deleteGroup = apity.path('/api/v1/group/{group}').method('delete').create();
export const updateGroup = apity.path('/api/v1/group').method('put').create();
export const createGroup = apity.path('/api/v1/event/{event}/groups').method('post').create();

export const createOrgSelfReg = apity.path("/api/v1/orgs/self_register").method('post').create();

export type Org = components["schemas"]["Org"];
export type Event = components["schemas"]["Event"];
export type EventOrg = components["schemas"]["EventOrg"];
export type OrgEvent = components["schemas"]["OrgEvent"];
export type Team = components["schemas"]["Team"];
export type NewTeam = components["schemas"]["NewTeam"];
export type Group = components["schemas"]["Group"];
