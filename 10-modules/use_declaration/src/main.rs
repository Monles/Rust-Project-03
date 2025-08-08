// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();
}

/*


Line 28 (function();) is called after leaving the block because of Rust's scoping rules for use statements:

Inside the block, you have use crate::deeply::nested::function;, which shadows the outer function() with deeply::nested::function().
When you exit the block, the local use binding goes out of scope.
Outside the block, calling function(); refers to the original fn function() defined at the top of the file.
So:

Inside the block: function() calls deeply::nested::function().
Outside the block: function() calls the top-level function().
That’s why line 28 calls the outer function() only after leaving the block.


*/