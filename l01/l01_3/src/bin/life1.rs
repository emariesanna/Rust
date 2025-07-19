fn main() {
    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        assert_eq!(*r, 1);
    }
}