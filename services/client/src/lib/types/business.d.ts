export type BusinessData = {
	id: number;
	name: string;
	stars: number;
	type: string;
	lat: number;
	lon: number;
	closesAt: number;
	opensAt: number;
};

export type BusinessDataFull = BusinessData & {
	zipCode: string;
	averagePrice: number;
	description: string;
	email: string;
	phone: string;
};
