// Pre-generation script, runs after variables are assigned but before generation
// Break the project name out into its components and create game_name and macos_pkg variables
let project_name = variable::get("project-name");
let project_components = project_name.split("-");
let game_name = "";

for (component, count) in project_components {
    component[0] = component[0].to_upper();
    game_name += component;
    if count + 1 < project_components.len {
        game_name += " ";
    }
}
let macos_pkg = game_name;
macos_pkg.replace(" ", "");

let project_package = project_name;
project_package.replace("-", "_");

variable::set("game_name", game_name);
variable::set("macos_pkg", macos_pkg);
variable::set("project_package", project_package);
