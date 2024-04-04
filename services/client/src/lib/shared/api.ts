import messageStore from '@/stores/message';
import type { BusinessData } from '@/types/business';
import type { LatLngExpression } from 'leaflet';

const BASE_URL = 'http://localhost:3000';

type Methods = 'GET' | 'PUT' | 'POST' | 'DELETE';

const request = async <T = object, R = object>(
	path: string,
	method: Methods = 'GET',
	data?: T,
	headers = {}
): Promise<R | null> => {
	try {
		const res = await fetch(`${BASE_URL}${path}`, {
			method,
			body: JSON.stringify(data),
			headers: {
				'Content-Type': 'application/json',
				...headers
			}
		});
		const contentType = res.headers.get('Content-Type');

		if (res.status === 500 || !contentType?.includes('application/json')) {
			messageStore.update(() => 'Something went wrong!');
			return null;
		}
		const json = await res.json();
		if (!res.ok) {
			messageStore.update(() => {
				return json.message || json.error || 'Failed to fetch a request';
			});
			return null;
		}
		return json;
	} catch (error: unknown) {
		messageStore.update(() => (error as Error).message);
		return null;
	}
};

export const searchBusinesses = async (pos: LatLngExpression, rad = 2000) => {
	if (!Array.isArray(pos)) return;
	const [lat, lon] = pos;
	const res = await request<null, { businesses: BusinessData[] }>(
		`/search?lon=${lon}&lat=${lat}&radius=${rad}`
	);
	return res?.businesses;
};

export const getById = async (id: number) => {
	const res = await request<null, { data: BusinessData }>(`/api/businesses/${id}`);
	return res?.data;
};

export const updateBusiness = async (id: number, data: Partial<BusinessData>) => {
	const res = await request<Partial<BusinessData>, { message: string }>(
		`/api/businesses/${id}`,
		'PUT',
		data
	);
	return res?.message;
};

export const createBusiness = async (data: Omit<BusinessData, 'id'>) => {
	const res = await request<Omit<BusinessData, 'id'>, { id: number }>(
		`/api/businesses`,
		'POST',
		data
	);
	return res?.id;
};
