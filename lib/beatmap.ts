enum EventType {
  TAP = 0,
  HOLD = 1,
  BOMB = 3,
}

interface BaseBeatmapEvent {
  time: number;
  slot: number;
}

interface TapEvent extends BaseBeatmapEvent {
  eventType: EventType.TAP;
}

interface HoldEvent extends BaseBeatmapEvent {
  eventType: EventType.HOLD;
  duration: number;
}

interface BombEvent extends BaseBeatmapEvent {
  eventType: EventType.BOMB;
}

type BeatmapEvent = TapEvent | HoldEvent | BombEvent;

type Beatmap = {
  id: number;
  title: string;
  artist: string;
  profiles: {
    email: string;
    id: string;
  };
  difficulty: number;
  bpm: number;
  offset: number;
  song_ref: string;
  events: BeatmapEvent[];
};

export type {
  Beatmap,
  BeatmapEvent,
  BombEvent,
  EventType,
  HoldEvent,
  TapEvent,
};

export function parseEvents(events: string): BeatmapEvent[] {
  return events.split("\n").map((event) => {
    const [time, slot, eventType, duration] = event.split(",").map(Number);
    switch (eventType) {
      case EventType.TAP:
        return { time, slot, eventType };
      case EventType.HOLD:
        return { time, slot, eventType, duration };
      case EventType.BOMB:
        return { time, slot, eventType };
      default:
        throw new Error(`Invalid event type: ${eventType}`);
    }
  });
}

export function toEventsString(events: BeatmapEvent[]): string {
  return events
    .map((event) => {
      switch (event.eventType) {
        case EventType.TAP:
          return `${event.time},${event.slot},${event.eventType}`;
        case EventType.HOLD:
          return `${event.time},${event.slot},${event.eventType},${event.duration}`;
        case EventType.BOMB:
          return `${event.time},${event.slot},${event.eventType}`;
      }
    })
    .join("\n");
}
