// place files you want to import through the `$lib` alias in this folder.
//
export interface BusTime {
  hour: number;
  minute: number;
}

export interface Weather {
  hourly: WeatherHour[];
  uv: number;
}

export interface WeatherHour {
  hour: string;
  temperature: number;
  rain: number;
  cloudcover: number;
}
