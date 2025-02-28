type WeekDay = "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun";

export interface Timeline {
  days: Record<WeekDay, Array<string>>;
}
