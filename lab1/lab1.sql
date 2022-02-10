/*
   Fill in the queries between the "SELECT QUERY#" and "SELECT END",
   see 'Query 0' for a sample answer.
   If there are any uncertainties contact Oliver and Oscar.

   Structure your queries in a readable format, you do not have
   to use capitalized keywords - Just be consistent.

   You must use joins when filtering, hard coded ids and similar are
   not acceptable. Avoid using natural joins for clarity.
   
   You are not allowed to use temporary tables.
*/

SELECT 'Query 0'; -- Do not remove
-- Select the product names of all products
SELECT 
    product_name 
FROM products;

SELECT 'END'; -- Do not remove

SELECT 'Query 1'; -- Do not remove
/* 
    Select the product names and unit prices of all products supplied by 
    supplier 'Tokyo Traders'.
*/
SELECT 
    products.product_name, products.unit_price 
FROM 
    products, suppliers
WHERE 
    products.supplier_id=suppliers.supplier_id 
    AND suppliers.company_name='Tokyo Traders';

SELECT 'END'; -- Do not remove


SELECT 'Query 2'; -- Do not remove
/* 
    Select the product names, category names and units in stock of all
    products supplied by suppliers based in 'Australia', 'Canada' or 'France'.
    NOTE: You are not allowed to use the 'OR' keyword.
*/
SELECT 
    products.product_name, categories.category_name, products.units_in_stock
FROM 
    products
INNER JOIN 
    suppliers
    ON 
        products.supplier_id=suppliers.supplier_id 
        AND suppliers.country 
    IN ('Australia','Canada','France') 
INNER JOIN 
    categories 
    ON 
        products.category_id=categories.category_id;

SELECT 'END'; -- Do not remove


SELECT 'Query 3'; -- Do not remove
/* 
   Select the product names and units on order of all products supplied 
   by the country of 'USA', that are not of the category 'Beverages', 
   ordered by the units on order descending.
*/ 
SELECT 
    products.product_name, products.units_on_order
FROM 
    products
INNER JOIN 
    suppliers
    ON 
        products.supplier_id=suppliers.supplier_id 
        AND suppliers.country = 'USA' 
INNER JOIN 
    categories 
    ON 
        products.category_id=categories.category_id 
        AND NOT categories.category_name='Beverages' 
ORDER BY 
    products.units_on_order 
DESC;


SELECT 'END'; -- Do not remove


SELECT 'Query 4'; -- Do not remove
/*
    Select the first names of the 5 youngest employees at the time of hire,
    ordered by their age at the time in ascending order.
*/
SELECT
    employees.first_name
FROM
    employees 
ORDER BY
    employees.birth_date 
DESC 
LIMIT 5;

SELECT 'END'; -- Do not remove


SELECT 'Query 5'; -- Do not remove
/*
    Select the number of orders shipped by each shipper and the shipper company 
    name, grouped by shipper and ordered by the number of orders for 
    each shipper in descending order.
*/
SELECT 
    COUNT(*), shippers.company_name 
FROM 
    orders 
INNER JOIN 
    shippers 
    ON 
        orders.ship_via=shippers.shipper_id 
GROUP BY 
    shippers.shipper_id 
ORDER BY 
    COUNT(*) DESC;

SELECT 'END'; -- Do not remove


SELECT 'Query 6'; -- Do not remove
/*
    Select the first name of employees that has never had an order made during 
    the months of November or December shipped later than the required date.
*/
SELECT 
    employees.first_name
FROM 
    employees 
WHERE 
    employees.first_name 
    NOT IN 
        (SELECT 
            employees.first_name 
        FROM 
            employees 
        WHERE 
            employees.employee_id 
            IN 
                (SELECT 
                    orders.employee_id 
                FROM 
                    orders
                WHERE 
                    ((EXTRACT(MONTH FROM orders.order_date) = 12) 
                    OR (EXTRACT(MONTH FROM orders.order_date) = 11)) 
                    AND orders.shipped_date > orders.required_date)
        );

SELECT 'END'; -- Do not remove


SELECT 'Query 7'; -- Do not remove
/*
    Select the names of all the products that have been ordered alongside the 
    product 'Ipoh Coffee', i.e. where there exists an order with both the 
    product and ipoh coffee ('Ipoh Coffee' should not be included in the 
    answer). There should be no repeats of product names in your answer.
*/
SELECT 
    product_name 
    FROM 
        products 
    WHERE 
        product_name!='Ipoh Coffee' 
        AND product_id 
        IN (
            SELECT 
                product_id 
            FROM   
                order_details
            WHERE order_id 
            IN (
                SELECT 
                    order_id 
                FROM 
                    order_details 
                WHERE 
                    product_id 
                IN (
                    SELECT 
                        product_id 
                    FROM 
                        products 
                    WHERE 
                        product_name='Ipoh Coffee'
                )
            )
        );

SELECT 'END'; -- Do not remove


SELECT 'Query 8'; -- Do not remove
/*
    Select the first and last names of managers as a single column separated by a space,
    the first and last name of their top employee as a single column separated by a space
    in terms of sales (in unit price, accounting for potential discount) 
    during the year 1996, along with the total revenue of the sales of the employee rounded
    down to the nearest integer, ordered by the total revenue. 
    
    Hint: Discount is given as a percentage. '0.2' means that the customer got a 20% discount
          on the purchase.
    Hint: Use `reports_to`

    The expected column format of the output is (header names of columns are not important):
    |first_name_manager last_name_manager|first_name_employee last_name_employee|revenue_by_employee|
    |-----------------------------------------------------------------------------------------------|
    | Magnus Anderson                    | Emma Employee                        | 300               |
    | Alice Svensson                     | Torstein Employed                    | 200               |
*/

SELECT m_name,e_name,rev FROM ((SELECT 
    CONCAT(managers.first_name, ' ', managers.last_name) AS m_name, 
    CONCAT(employees.first_name, ' ', employees.last_name) AS e_name,
    FLOOR(SUM(unit_price*quantity*(1-discount))) AS rev
FROM 
    orders,order_details,employees, employees AS managers
WHERE 
    managers.employee_id=employees.reports_to
    AND employees.employee_id=orders.employee_id
    AND order_details.order_id=orders.order_id
    AND EXTRACT(YEAR FROM orders.order_date)=1996
GROUP BY 
    managers.first_name, managers.last_name, employees.first_name, employees.last_name
ORDER BY 
    FLOOR(SUM(unit_price*quantity*(1-discount))) 
DESC) AS a
INNER JOIN
(SELECT MAX(rev) FROM (SELECT 
    employees.reports_to, employees.employee_id, FLOOR(SUM(unit_price*quantity*(1-discount))) AS rev
FROM 
    orders,order_details,employees
WHERE 
    employees.employee_id=orders.employee_id
    AND order_details.order_id=orders.order_id
    AND EXTRACT(YEAR FROM orders.order_date)=1996
GROUP BY 
    employees.employee_id,employees.reports_to) AS r GROUP BY reports_to) AS b
ON a.rev=b.max);

SELECT 'END'; -- Do not remove


 
SELECT 'Query 9'; -- Do not remove
/*
    Select the company names of the customers that have made at least one order
    handled by each of the employees. I.E. all employees has been responsible
    for at least one order made by the customer.
*/
SELECT 
    customers.company_name 
FROM 
    customers, orders, employees 
WHERE 
    customers.customer_id=orders.customer_id 
GROUP BY 
    customers.company_name 
HAVING 
    COUNT(DISTINCT orders.employee_id) = 
    COUNT(DISTINCT employees.employee_id);

SELECT 'END'; -- Do not remove



SELECT 'Query 10'; -- Do not remove
/*
    Two products are related if they exist in an order together. Two products are also related if they
    have been on an order with a product that 'connects them'.

    Example:
    In the example below, apples and pears are related as they exist on the same order. Apples and
    cheese are also related, as one of the related products of apples are related to cheese. 
    The sets of related products for orders below are thus as follows:

    Note: Orders and producs below are part of your database state.

            Candy
            Sugar
        Order 5:
            Gingerbread
            Sugar

    Related products:
        Apples -> (Pears, Cheese)
            Apples -> Pears
                Pears -> Cheese
            Pears
        Order 2:
            Pears
            Cheese
        Order 3:
            Soda
            Candy
        Order 4:
            Candy
            Sugar
        Order 5:
            Gingerbread
            Sugar

    Related products:
        Apples -> (Pears, Cheese)
            Apples -> Pears
                Pears -> Cheese

        Pears  -> (Apples, Cheese)
            Pears -> Apples
            Pears -> Cheese

        Cheese -> (Pears, Apples)
            Cheese -> Pears
                Pears -> Apples

        Candy -> (Sugar, Soda, Gingerbread)
            Candy -> Sugar
                Sugar -> Gingerbread
            Candy -> Soda
        
        Soda -> (Candy, Sugar, Gingerbread)
            Soda -> Candy
                Candy -> Sugar
                    Sugar -> Gingerbread



    Question:
        Create a query that returns the product names of all related
        products of an input product name based on orders,
        ordered by product names ascending. The base product should not be included.

        Call your query with the base product 'Sugar'.
    
        Note: You must use recursion.
 */

SELECT product_name FROM products WHERE product_id IN (
    WITH RECURSIVE R AS (
            (
                SELECT order_id,product_id
                FROM order_details
                WHERE
                product_id IN (
                    SELECT product_id FROM products WHERE product_name='Sugar'
                )
            )
            UNION 
            (
                SELECT related.order_id, related.product_id
                FROM order_details AS related
                JOIN R ON 
                    (R.product_id = related.product_id) OR 
                    (R.order_id = related.order_id)
            ) 
    )
    SELECT DISTINCT R.product_id
    FROM R WHERE R.product_id NOT IN (
        SELECT product_id FROM products WHERE product_name='Sugar'
    )
);

SELECT 'END'; -- Do not remove
