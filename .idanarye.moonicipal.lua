local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

local _, cfg = moonicipal.import(require'idan.project.rust')

cfg.cli_args_for_targets.subqueue = {
    as_json = {'json', 'https://www.astralcodexten.com'},
    as_html = {'html', 'https://www.astralcodexten.com'},
}
