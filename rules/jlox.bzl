def _impl(ctx):
    script = """
#!/usr/bin/env bash
{command} {cmdline_args}
if [ $? -eq {expected_retcode} ]; then
    exit 0
else
    echo "Got the wrong return code"
    exit 1
fi
""".format(
        command = ctx.executable._jlox.short_path,
        expected_retcode = ctx.attr.expected_retcode,
        cmdline_args = " ".join(ctx.attr.cmdline_args),
    )

    ctx.actions.write(output = ctx.outputs.executable, content = script)

    runfiles = ctx.runfiles(files = [ctx.executable._jlox])
    return [DefaultInfo(runfiles = runfiles)]


jlox_test = rule(
    implementation = _impl,
    attrs = {
        "expected_retcode" : attr.int(
            default=0,
            doc="The expected return code when running the jlox interpreter",
            mandatory=False),
        "cmdline_args": attr.string_list(
            mandatory=False,
            allow_empty=True,
            default=[],
            doc="Arguments passed to the interpreter executable"),
        "_jlox": attr.label(
            default = Label("//jlox:jlox-rs"),
            allow_single_file = True,
            executable = True,
            cfg = "exec",
        ),
    },
    test = True,
    doc="Test running the interpreter"
)


def _parallel_executable_test_impl(ctx):
    # Shard the input files based on the test sharding parameters
    jlox_executable = Label("//jlox:jlox-rs")
    shard_count = ctx.attr.shard_count
    inputs_per_shard = len(ctx.attr.lox_inputs) // shard_count

    # Generate a list of actions for the files in the current shard
    actions = []
    for input_file in ctx.attr.lox_inputs[0:5]:
        action_name = "{}_{}".format(jlox_executable, input_file)
        output_file = "{}_output.txt".format(action_name)
        command = "./{} {} > {}".format(jlox_executable, input_file, output_file)
        action = ctx.actions.declare_file(
            action_name,
            inputs = ctx.files.lox_inputs,
            outputs = [output_file],
            command = command,
        )
        actions.append(action)

    # Run all actions in parallel
    ctx.actions.run()

# Bazel _test rule for parallel execution of an executable with sharded input files
parallel_executable_test = rule(
    implementation = _parallel_executable_test_impl,
    attrs = {
        "lox_inputs": attr.label_list(default=[],
                                      allow_files=True,
                                      allow_empty=True)
    },
    test = True,
)
