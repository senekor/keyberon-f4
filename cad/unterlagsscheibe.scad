$fn = 100;

difference()
{
    union()
    {
        cylinder(h = 1, d = 6);
        cylinder(h = 2.4, d = 3.7);
    }
    cylinder(h = 20, d = 2.3, center = true);
}
