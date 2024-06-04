init game

-> meta commands
    - exit

-> show Welcome screen.
    - join player (toggle on press short action)
    - start game (press long action)
    - show input instructions
    - show objectives
    - show story?
    - show leaderboard (1 player / 2 players)

-> show game screen
    - spring
    - 6 students in class (R0C0,R0C1,R0C2,R1C0,R1C1,R1C2) and 3 desks empty (R2C0,R2C1,R2C2). No lesson
    - 1 monster revealed (1 need = autumn)
    - 1 monster hidden (1 need)

-> teacher actions
    - move to stations
    - short + long

-> durations
    - season: 15s
    - move teacher: 2s
    - [s] teach: 5s
    - [l] graduate: 10s
    - [s] welcome: 5s
    - [l] recruit: 10s
    - [s] watch: 5s
    - [s] defend: 5s
    - [l] repair: 10s
    - [s] eat: 5s
    - [l] cook: 10s
    - monster wait: dynamic
    - monster attack: dynamic

-> algo for new students
    - only if classroom not full
    - classroom less than 3 students
    - either a monster is fulfilled
    - either every season with a random (20% chance)
    - goes to the next random available space (front row or behind student)

-> algo for monster
    - difficulty = nb monsters defeated
        - 1 => need=autumn, wait=40s, attack=60s
        - 2 => need=spring, wait=40s, attack=60s
        - 3 => need=summer+winter, wait=40s, attack=60s
        - 4 => need=winter, wait=40s, attack=60s
        - 5 => need=autumn+winter+spring, wait=30s, attack=60s
        - n<10 => need=random(1,2,3 needs), wait=30s, attack=60s
        - n<15 => need=random(1,2,3 needs), wait=20s, attack=30s
        - n<20 => need=random(1,2,3 needs), wait=15s, attack=30s
        - n<25 => need=random(2,3 needs), wait=15s, attack=30s
        - n<30 => need=random(2,3 needs), wait=10s, attack=30s
        - n<35 => need=random(3 needs), wait=10s, attack=30s
        - n<50 => need=random(3 needs), wait=10s, attack=10s
        - n>=50 => need=random(3 needs), wait=5s, attack=10s
    - if no monster, add monster

-> score:
    - 1 point per student taught
    - 10 points per student graduated
    - 20 points per monster fulfilled