# Insert Game Title Here
Pending because Kuumin sucks at naming lmao.

## What is this?
A smoothie

## Logs
<a name="Lg-2022/10/23-Dreamingtown-00"></a>
**Lg-2022/10/23-Dreamingtown-00**
Still doing some house-cleaning.
Moved the constants to their own .rs file.
Still reworking on input parser; don't know if the new way of parsing will work as intended. But we'll see. [Refer to this](#Nt-2022/10/23-Dreamingtown-00).
Also added this README because i can. Maybe it will be useful

<a name="Lg-2022/10/23-KuuminKochi-00"></a>
**Lg-2022/10/23-KuuminKochi-00**
Working on the enemy bullet generator right now. Will have to reference this a lot:
https://matthew-brett.github.io/teaching/rotation_2d.html
While we're on the topic, i'll have to compose a little tune for the game, which I will reserve at a later time.
Currently, I'm trying to figure out how to make the bullets shoot from all direction. I've already implemented the rotation 2D. It'll be a lil funky if it moves in the x-axis after being spawned

<a name="Lg-2022/10/23-KuuminKochi-01"></a>
**Lg-2022/10/23-KuuminKochi-01**
Added collision update, segregated some things like enemy spawning and player bullet spawn. Overall, the code is much cleaner now. drreamingtown is now a femboy

<a name="Lg-2022/10/25-KuuminKochi-00"></a>
**Lg-2022/10/25-KuuminKochi-01**
Working on the math for the bullet hell patterns, over. No code but yes math

## Notes
<a name="Nt-2022/10/23-Dreamingtown-00"></a>
**Nt-2022/10/23-Dreamingtown-00**
Current assumption: KeyboardInput reads everything including unpressed buttons
Untested, need to check again later.
Left this note in case something goes wrong related to the input parser

## TODOs (that we currently have in mind)
- [ ] 0. Clean-up until i (dreamingtown) think it's good enough
- [ ] 1. Test input parser
- [ ] 2. Test collider system
- [ ] 3. Make the bullets move relative to where they're spawned
- [ ] 4. Add bullet curve
- [ ] 5. Add bullet offsets (relative to the enemy)
- [ ] 6. Add firerate
- [ ] 7. Create spellcards

## README format
### Logs
ID: Lg-YYYY/MM/DD-Writer-Number
Content: Infomal, type whatever.
Example: Lg-2021/02/06-Koomingtime-01
### Notes
ID: Nt-YYYY/MM/DD-Writer-Number
Content: Infomal, type whatever.
Example: Nt-2021/04/31-Dream-03
### TODOs
ID: Unique incrementing ID

