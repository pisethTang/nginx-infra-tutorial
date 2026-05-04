1. Mini Lisp interpreter 

<i>This is the classic Scheme project.

You build a small interpreter that evaluates expressions like:

(+ 1 2)
(define x 10)
(* x 3)
(if (> x 5) "big" "small")

Your interpreter would support:

numbers
symbols
variables
define
lambda
if
function calls
environments

This teaches the deepest Scheme ideas:

evaluation, environments, closures, recursion, homoiconicity

AI angle: later, you can make an AI explain why an expression evaluates a certain way, step by step.

Example:

Expression:
((lambda (x) (+ x 1)) 5)

AI explanation:
1. The lambda creates a function.
2. The function receives x = 5.
3. The body becomes (+ 5 1).
4. Result is 6.

This is probably the best “computer science fundamentals” project.
</i>




2. Symbolic differentiator

<i>
Input:

(derivative '(* x x) 'x)

Output:

(+ (* x 1) (* x 1))

Then simplify it to:

(* 2 x)

Support expressions like:

(+ x 3)
(* x x)
(sin x)
(exp x)
(^ x 2)

This teaches:

pattern matching style thinking, recursion over trees, symbolic expressions, quotation

AI angle: user types a natural language maths problem, AI converts it into symbolic Scheme form.

Example:

User: "Differentiate x squared plus 3x"
AI converts to:
(+ (^ x 2) (* 3 x))

Scheme computes:
(+ (* 2 x) 3)

</i>



Best first project: menu query engine

Start with this because it is small, useful, and teaches the right ideas.

### Your first version should support:

add menu item
list all items
filter by category
filter by max price
filter by allergen exclusion
filter by dietary tag
find cheapest item
find items containing ingredient

Then add AI later.





A simple architecture could be:

User text
   ↓
AI converts text into structured query
   ↓
Scheme validates and executes query
   ↓
AI turns result into natural language response

Example:

User:
"Do you have anything vegetarian under $20 without dairy?"

AI converts:
(query
  (dietary vegetarian)
  (max-price 20)
  (exclude-allergens dairy))

Scheme result:
(("Avocado Toast" 14.50)
 ("Mushroom Bowl" 18.00))

AI response:
"Yes — you could get the Avocado Toast for $14.50 or the Mushroom Bowl for $18."

The key idea: do not let AI decide the truth. Let AI translate language. Let Scheme check facts and rules.


REPL: Read Evaluate Print Loop 
