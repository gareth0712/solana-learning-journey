type Meal = 
  | { type: "FishAndChips"; chip_proportion: number }
  | { type: "Hamburger"; vegetarian: boolean };

const meal: Meal = { type: "Hamburger", vegetarian: true };

// TypeScript does not enforce pattern matching, but we can use a type guard ourselves
if (meal.type === "Hamburger") {
  const { vegetarian } = meal;
  console.log("I had a hamburger!");
  if (vegetarian) {
    console.log("It was a vegetarian hamburger.");
  } else {
    console.log("It was a non-vegetarian hamburger.");
  }
} else if (meal.type === "FishAndChips") {
  const { chip_proportion } = meal;
  console.log("I had fish and chips with a chip proportion of:", chip_proportion);
}