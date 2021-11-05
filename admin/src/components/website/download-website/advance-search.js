import React from "react";
import { Form, Input, Checkbox, Row, Col } from "antd";

const AdvanceSearch = () => {
  return (
    <React.Fragment>
      <Form>
        <Checkbox.Group style={{ width: "100%" }}>
          <Row gutter={[12, 12]}>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="Recursive">Recursive</Checkbox>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="No Clobber">No Clobber</Checkbox>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="Page Requisites">Page Requisites</Checkbox>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="Convert Links">Convert Links</Checkbox>
              </Form.Item>
            </Col>
            <Col span={4}>
              <Form.Item>
                <Checkbox value="HTML Extension">HTML Extension</Checkbox>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="For Windows">For Windows</Checkbox>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="No Parent">No Parent</Checkbox>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="Continue">Continue</Checkbox>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item>
                <Checkbox value="Span Hosts">Span Hosts</Checkbox>
              </Form.Item>
            </Col>
            <Col span={4}>
              <Form.Item>
                <Checkbox value="Ignore Robot">Ignore Robot</Checkbox>
              </Form.Item>
            </Col>
            <Col span={4}>
              <Form.Item>
                <Checkbox value="Domain ">Domain </Checkbox>
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item>
                <Input size="small" />
              </Form.Item>
            </Col>
            <Col span={4}>
              <Form.Item>
                <Checkbox value="Referer">Referer </Checkbox>
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item>
                <Input size="small" />
              </Form.Item>
            </Col>
            <Col span={4}>
              <Form.Item>
                <Checkbox value="Limit Speed">Limit Speed</Checkbox>
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item>
                <Input size="small" />
              </Form.Item>
            </Col>
          </Row>
        </Checkbox.Group>
      </Form>
    </React.Fragment>
  );
};

export default AdvanceSearch;
