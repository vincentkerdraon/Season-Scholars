Game name: Season Scholars

-> meta commands
    - exit

-> Welcome screen.
    - goal
```
You play as a wise teacher guiding farmers through the changing seasons. Each season brings new crops to learn: asparagus, cherries, chanterelles and lemons. But beware! Monsters with unique needs threaten your village. To protect your people, you must graduate students with the precise knowledge to combat these threats. Will you rise to the challenge?
```
    - howto
```
- Move your teacher from station to station by selecting a direction and pressing "short action."
- In front of the students, you can teach them about the current season (short action) or graduate them.
- At the door in the back, you can welcome a new student (short action) or find a new one (long action).
- Through the magical windows on the left, you can spy on incoming monsters to learn their needs (short action).
- You can defend (short action) the monster portal on the left or repair it (long action).
- At the cooking station, you can eat (short action) or cook (long action).
```
    - show input mapping (see images: arcade + keyboard)
    - join + start game 
```
press "short action" to join/leave
press "long action" to start the game
press "reset" to exit
```

-> Game over screen
    - reason of death
    - score and duration of the game
    - one of these
        - `The village falls as the monsters overrun the fields. Better luck next time, scholar.`
        - `The students couldn't keep up with the monsters' demands. Try again and teach them better!`
        - `The knowledge wasn't enough to protect the village. The monsters have won this season.`
        - `Despite your efforts, the monsters have breached the village. Prepare for the next challenge.`
        - `Your teachings couldn't save the village this time. Study hard and try again!`
        - `The crops are trampled, and the village is lost. Will you return to save it?`
        - `The monsters have conquered the village. Can you devise a better strategy next time?`
        - `The students weren't ready, and the village has fallen. Regroup and educate them anew.`
        - `Your defense failed, and the monsters reign supreme. Can you rise to the challenge next season?`
        - `The village is lost to the monster horde. Sharpen your knowledge and return for revenge.`

-> game screen
    - spring
    - 6 students in class (R0C0,R0C1,R0C2,R1C0,R1C1,R1C2) and 3 desks empty (R2C0,R2C1,R2C2). No lesson
    - 1 monster revealed (need = autumn)
    - 1 monster hidden (need=spring)

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
    - [l] repair: 10s
    - [s] eat: 5s
    - [l] cook: 10s
    - monster wait: dynamic
    - monster attack: dynamic

-> algo for new students
    - only if classroom not full
    - classroom less than 3 students
    - a monster is fulfilled
    - every season with a random (20% chance)
    - student goes to the next random available space (front row or behind student)

-> algo for monster
    - monsters arrive every season, they wait in line until they reach the portal. At the portal, they wait a moment hidden, then become visible. They attack if they are visible for too long and damage the door. The door can hold a few hits, then game over.
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

-> Buttons
(following game jam guidelines + special additions for keyboard )
    - common
        - Reset: mouse 3 || escape
    - player1
        - Move up: arrow up 
        - Move left: arrow left
        - Move down: arrow down
        - Move right: arrow right
        - Short action: shift left || shift right
        - Long action: ctrl left || ctrl right
    - player2
        - Move up: R || 8
        - Move left: D || 4
        - Move down: F || 2
        - Move right: G || 6
        - Short action: Q || 0 || - || Page Up
        - Long action: A || W || . || + || Page Down