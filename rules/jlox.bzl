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
)
