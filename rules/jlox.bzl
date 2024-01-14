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
    # TODO make jlox interpreter run across all files
    # TODO parallelise the run
    jlox_executable = Label("//jlox:jlox-rs")
    script = """
#!/usr/bin/env bash
{command} {cmdline_args}
if [ $? -eq 0 ]; then
    exit 0
else
    echo "Got the wrong return code"
    exit 1
fi
"""
    files_depset = ctx.attr.lox_inputs[0]

    # Iterate over the depset
    res = []
    for inp in files_depset.files.to_list():
        # print(inp)
        named_scr = script.format(
            command = jlox_executable,
            cmdline_args = inp
        )

        ctx.actions.write(output = ctx.outputs.executable, content = named_scr)

        runfiles = ctx.runfiles(files = [ctx.executable._jlox])
        # res.append(DefaultInfo(runfiles = runfiles))
    return res


# Bazel _test rule for parallel execution of an executable with sharded input files
parallel_executable_test = rule(
    implementation = _parallel_executable_test_impl,
    attrs = {
        "lox_inputs": attr.label_list(default=["@bobs_test_suite//:exported_testdata"],
                                      allow_files=True,
                                      allow_empty=False),
        "_jlox": attr.label(
            default = Label("//jlox:jlox-rs"),
            allow_single_file = True,
            executable = True,
            cfg = "exec",
        ),

    },
    test = True,
)
