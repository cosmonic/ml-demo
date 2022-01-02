// mlinference.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "com.pervaisive.interfaces.mlinference", crate: "mlinference" } ]

namespace com.pervaisive.interfaces.mlinference

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U8
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Mlinference service 
@wasmbus(
    contractId: "example:interfaces:mlinference",
    actorReceive: true,
    providerReceive: true )

service Mlinference {
  version: "0.1",
  operations: [ Calculate, Load, InitExecutionContext ]
}

/// Calculates the factorial (n!) of the input parameter
operation Calculate {
  input: U32,
  output: U64
}

/// load
operation Load {
  input: LoadInput,
  output: LoadResult
}

/// init_execution_context
operation InitExecutionContext {
  input: Graph,
  output: IecResult
}

structure LoadInput {
    @required
    builder: GraphBuilder,

    @required
    encoding: GraphEncoding,

    @required
    target: ExecutionTarget
}

// list GraphBuilderArray {
//   member: GraphBuilder
// }

list GraphBuilder {
  member: U8
}

structure GraphEncoding {
  // enum seems to have no impact on the code generator
  @enum([
    {
        value: 0,
        name: "OPENVINO",
        documentation: """TBD""",
        tags: ["graphEncoding"]
    },
    {
        value: 1,
        name: "ONNX",
        documentation: """TBD""",
        tags: ["graphEncoding"]
    }
  ])
  @required
  encoding: U8
}

structure ExecutionTarget {
  // enum seems to have no impact on the code generator
  @enum([
    {
      value: 0,
      name: "EXECUTION_TARGET_CPU",
      documentation: """TBD""",
      tags: ["executionTarget"]
    },
    {
      value: 1,
      name: "EXECUTION_TARGET_GPU",
      documentation: """TBD""",
      tags: ["executionTarget"]
    },
    {
      value: 2,
      name: "EXECUTION_TARGET_TPU",
      documentation: """TBD""",
      tags: ["executionTarget"]
    }
  ])
  @required
  target: U8
}

structure Graph {
  @required
  graph: U32
}

@error("client")
structure GuestError {
  // enum seems to have no impact on the code generator
  @enum([
    {
      value: 0,
      name: "MODEL_ERROR",
      documentation: """TBD""",
      tags: ["MlInferenceError"]
    },
    {
      value: 1,
      name: "INVALID_ENCODING_ERROR",
      documentation: """TBD""",
      tags: ["MlInferenceError"]
    },
  ])
  @required
  modelError: U8
}

@error("server")
structure RuntimeError {
  // enum seems to have no impact on the code generator
  @enum([
    {
      value: 0,
      name: "RUNTIME_ERROR",
      documentation: """TBD""",
      tags: ["MlInferenceError"]
    },
    {
      value: 1,
      name: "OPEN_VINO_ERROR",
      documentation: """TBD""",
      tags: ["MlInferenceError"]
    },
    {
      value: 2,
      name: "ONNX_ERROR",
      documentation: """TBD""",
      tags: ["MlInferenceError"]
    },
  ])
  @required
  runtimeError: U8
}

structure LoadResult {
  hasError: Boolean,

  runtimeError: RuntimeError,

  guestError: GuestError,

  @required
  graph: Graph,
}

/// InitExecutionContextResult
structure IecResult {
  hasError: Boolean,

  runtimeError: RuntimeError,

  guestError: GuestError,

  @required
  gec: GraphExecutionContext,
}

structure GraphExecutionContext {
  @required
  gec: U32
}