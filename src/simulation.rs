use crate::{app::{App, Policy}, variable::VariableType};

pub fn step_simulation(app: &mut App) {
    app.time += 1.0;

    step_stability(app);
    step_crisis(app);
    step_opinion(app);
    step_expenses(app);
    step_tax(app);
    step_income(app);
    step_gdp(app);
}

fn step_gdp(app: &mut App) {
    let gdp = app.get_variable_mut(&VariableType::GDP);
    
    let new_gdp = gdp.value * 1.0025;
    gdp.update(new_gdp);
}

fn step_income(app: &mut App) {
    let gdp = app.get_variable(&VariableType::GDP).value;
    let tax = app.get_variable(&VariableType::Tax).value;
    let expenses = app.get_variable(&VariableType::Expenses).value;

    let income = app.get_variable_mut(&VariableType::Income);

    let new_income = gdp * (tax / 100.0) - expenses;
    income.update(new_income);
}

fn step_tax(app: &mut App) {
    let new_tax: f64 = match app.policy {
        Policy::Austerity => 20.0,
        Policy::Neutral => 15.0,
        Policy::Stimulus => 10.0,
    };
    let tax = app.get_variable_mut(&VariableType::Tax);

    tax.update(new_tax.clamp(0.0, 100.0));
}

fn step_expenses(app: &mut App) {
    let gdp = app.get_variable(&VariableType::GDP).value;
    let rate = match app.policy {
        Policy::Austerity => 0.05,
        Policy::Neutral => 0.1,
        Policy::Stimulus => 0.2,
    };

    let expenses = app.get_variable_mut(&VariableType::Expenses);

    let new_expenses = gdp * rate;
    expenses.update(new_expenses);
}

fn step_opinion(app: &mut App) {
    // Higher tax, lower expenses gives a lower opinion
    let tax = app.get_variable(&VariableType::Tax).value;
    let expenses = app.get_variable(&VariableType::Expenses).value;
    let gdp = app.get_variable(&VariableType::GDP).value;

    
    let opinion = app.get_variable_mut(&VariableType::Opinion);

    let new_opinion = opinion.value - (tax * 0.01) + (expenses / gdp);
    opinion.update(new_opinion.clamp(0.0, 100.0));
}

fn step_crisis(app: &mut App) {
    let time = app.time + 160.0; // Offset so crisis starts at 0.
    let crisis = app.get_variable_mut(&VariableType::Crisis);

    let new_crisis = f64::sin(time / 10.0) + f64::cos(time / 100.0) * 30.0;
    crisis.update(new_crisis.clamp(0.0, 100.0));
}

fn step_stability(app: &mut App) {
    // Negative income, low opinion or high crisis
    // gives a lower stability
    let income = app.get_variable(&VariableType::Income).value;
    let gdp = app.get_variable(&VariableType::GDP).value;
    let opinion = app.get_variable(&VariableType::Opinion).value;
    let crisis = app.get_variable(&VariableType::Crisis).value;
    let stability = app.get_variable_mut(&VariableType::Stability);

    let new_stability = stability.value + (income / gdp * 0.1) - (opinion * 0.001) - (crisis * 0.001);
    stability.update(new_stability.clamp(0.0, 100.0));
}
