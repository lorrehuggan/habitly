export type WeekDay = "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun";

export interface Habit {
  id: string;
  title: string;
  description: string;
  created: string;
  color: string;
  streak: number;
  category: string;
  status: string;
}

export interface Commit {
  id: string;
  created: string;
  message: string;
  status: "ongoing" | "completed";
  completions: number;
  habit_id: string;
}

export interface Timeline {
  days: Record<WeekDay, Array<string>>;
}
