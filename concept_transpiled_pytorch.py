# package SamplePackage

# Generate by visual editor
# edit this file by visual editor
#

# use("PyTorchPack")
import torch
import torch.nn as nn

class SampleModule(nn.Module):
    # properties
    ## kernel_size: List[int]
    ## stride: int

    # constructor
    # mode: stack
    def __init__(
        self,
        kernel_size  = [3,3],
    ):
        # initization
        super(SampleModule, self).__init__()
        self.params = {
            "kernel_size": kernel_size,
            "stride_size" : 1
        }

        # stack_0
        self.stack_0 = nn.Conv2d(
            in_channels=3,
            out_channels=64,
            kernel_size=kernel_size,
            stride=self.params["stride_size"],
            padding=1
        )

        # stack_1
        self.stack_1 = nn.MaxPool2d(
            kernel_size=self.params["kernel_size"],
        )

        # stack_2
        self.stack_2 = nn.CELU()

    def forward(self, input_tensor):
        output_tensor = input_tensor
        # main branch
        branch_0 = self.stack_0(output_tensor)
        branch_0 = self.stack_1(branch_0)
        branch_0 = self.stack_2(branch_0)
        return branch_0
