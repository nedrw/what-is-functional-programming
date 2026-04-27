// 演示：用纯函数实现状态机
// 核心思想：状态转换函数签名是 fn(State, Event) -> State，无副作用
// 优势：可测试、可审计、天然支持并发

#[derive(Debug, PartialEq, Clone)]
enum State {
    Idle,
    Processing { progress: u8 },
    Done,
    Failed(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Event {
    Start,
    Progress(u8),
    Complete,
    Error(String),
}

/// 核心转换函数 —— 纯函数
fn transition(state: State, event: &Event) -> State {
    match (state, event) {
        (State::Idle, Event::Start) => State::Processing { progress: 0 },
        (State::Processing { .. }, Event::Progress(p)) if *p <= 100 => {
            State::Processing { progress: *p }
        }
        (State::Processing { .. }, Event::Complete) => State::Done,
        (State::Processing { .. }, Event::Error(msg)) => State::Failed(msg.clone()),
        // 非法组合：保持当前状态不变
        (state, _) => state,
    }
}

fn main() {
    let events = vec![
        Event::Start,
        Event::Progress(30),
        Event::Progress(70),
        Event::Complete,
    ];

    // 纯函数计算最终状态（无副作用）
    let final_state = events.iter().fold(State::Idle, transition);

    println!("最终状态: {:?}", final_state);
    assert_eq!(final_state, State::Done);
    println!("\n✅ 状态机正确到达终态");

    // 如果需要打印转换过程，单独处理
    println!("\n转换轨迹：");
    let mut state = State::Idle;
    for event in &events {
        let new_state = transition(state.clone(), event);
        println!("  {:?} + {:?} → {:?}", state, event, new_state);
        state = new_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_from_idle_start() {
        assert_eq!(
            transition(State::Idle, &Event::Start),
            State::Processing { progress: 0 }
        );
    }

    #[test]
    fn test_transition_processing_progress() {
        assert_eq!(
            transition(State::Processing { progress: 50 }, &Event::Progress(75)),
            State::Processing { progress: 75 }
        );
    }

    #[test]
    fn test_transition_processing_complete() {
        assert_eq!(
            transition(State::Processing { progress: 100 }, &Event::Complete),
            State::Done
        );
    }

    #[test]
    fn test_transition_processing_error() {
        let result = transition(
            State::Processing { progress: 50 },
            &Event::Error("disk full".to_string()),
        );
        assert_eq!(result, State::Failed("disk full".to_string()));
    }

    #[test]
    fn test_transition_invalid_events_ignored() {
        // Idle 状态下收到 Progress 事件，应该保持 Idle
        assert_eq!(transition(State::Idle, &Event::Progress(50)), State::Idle);

        // Done 状态下收到 Start 事件，应该保持 Done
        assert_eq!(transition(State::Done, &Event::Start), State::Done);
    }

    #[test]
    fn test_full_sequence() {
        let events = vec![
            Event::Start,
            Event::Progress(30),
            Event::Progress(70),
            Event::Complete,
        ];

        let final_state = events.iter().fold(State::Idle, transition);
        assert_eq!(final_state, State::Done);
    }
}
