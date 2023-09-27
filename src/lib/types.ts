// place files you want to import through the `$lib` alias in this folder.
//
export interface BusTime {
  hour: number;
  minute: number;
}

export interface Weather {
  weather: WeatherHour[];
  uv: number;
}

export interface WeatherHour {
  hour: Date;
  temperature: number;
  rain: number;
  cloud: number;
}
